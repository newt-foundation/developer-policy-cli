// op-sim
// simulates operator execution of a wasm
//
// usage: op-sim <wasm_file> <input_json>

use clap::Parser;
use std::path::PathBuf;
use wasmtime::component::{bindgen, Component, HasSelf, Linker, ResourceTable};
use wasmtime::{Config, Engine};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

bindgen!({
    path: "wit",
    async: true,
});

use newton::provider::http::{HttpRequest, HttpResponse};

#[derive(Parser)]
#[command(name = "op-sim")]
#[command(about = "Simulate operator WASM execution")]
struct Cli {
    /// Path to the WASM component file
    wasm_file: PathBuf,
    /// Input JSON string
    input_json: String,
}

pub struct HttpProvider;

// custom provider
impl newton::provider::http::Host for HttpProvider {
    async fn fetch(&mut self, request: HttpRequest) -> Result<HttpResponse, String> {
        // WIT -> reqwest request
        let method = match request.method.as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            _ => return Err(format!("unsupported HTTP method: {}", request.method)),
        };

        let client = reqwest::Client::new();
        let mut req_builder = client.request(method, &request.url);

        // add headers
        for (key, value) in request.headers {
            req_builder = req_builder.header(&key, &value);
        }

        // add body if present
        if let Some(body) = request.body {
            req_builder = req_builder.body(body);
        }

        // execute request
        match req_builder.send().await {
            Ok(response) => {
                let status = response.status().as_u16();
                let headers = response
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
                    .collect();

                match response.bytes().await {
                    Ok(body) => Ok(HttpResponse {
                        status,
                        headers,
                        body: body.to_vec(),
                    }),
                    Err(e) => Err(format!("failed to read response body: {e}")),
                }
            }
            Err(e) => Err(format!("HTTP request failed: {e}")),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let input = &cli.input_json;

    // Read WASM file
    let wasm_bytes = std::fs::read(&cli.wasm_file)?;

    let mut wasm_config = Config::new();
    wasm_config.wasm_component_model(true);
    wasm_config.async_support(true);

    wasm_config.consume_fuel(true);
    wasm_config.max_wasm_stack(1024 * 1024 * 16); // 16 MiB
    wasm_config.async_stack_size(1024 * 1024 * 32); // 32 MiB

    let engine = Engine::new(&wasm_config)?;
    let component = Component::new(&engine, wasm_bytes)?;

    struct MyCtx {
        table: ResourceTable,
        wasi: WasiCtx,
        http: HttpProvider,
        wasi_http_ctx: WasiHttpCtx,
    }

    impl IoView for MyCtx {
        fn table(&mut self) -> &mut ResourceTable {
            &mut self.table
        }
    }

    impl WasiView for MyCtx {
        fn ctx(&mut self) -> &mut WasiCtx {
            &mut self.wasi
        }
    }

    impl WasiHttpView for MyCtx {
        fn ctx(&mut self) -> &mut WasiHttpCtx {
            &mut self.wasi_http_ctx
        }
    }

    // wasi context (exact same as operator)
    let ctx = MyCtx {
        table: ResourceTable::new(),
        wasi_http_ctx: WasiHttpCtx::new(),
        http: HttpProvider,
        wasi: WasiCtxBuilder::new()
            .args(&["plugin", input]) // pass input as command line argument
            .inherit_stdin()
            .inherit_stdout()
            .inherit_stderr()
            .inherit_network() // allow network access (eg for HTTP requests)
            .build(),
    };

    let mut store = wasmtime::Store::new(&engine, ctx);

    // Set fuel (same as operator default)
    store.set_fuel(2_000_000)?;

    let mut linker = Linker::new(&engine);
    linker.allow_shadowing(true);

    // add wasi functions to linker
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

    // add wasi http to linker
    wasmtime_wasi_http::add_to_linker_async(&mut linker)?;

    // add the newton provider interface to linker
    newton::provider::http::add_to_linker::<MyCtx, HasSelf<HttpProvider>>(
        &mut linker,
        |ctx: &mut MyCtx| &mut ctx.http,
    )?;

    // instantiate the newton provider world
    let newton_provider =
        NewtonProvider::instantiate_async(&mut store, &component, &linker).await?;

    // execute the run function with JSON input
    let result = newton_provider.call_run(&mut store, input).await?;

    match result {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        }
        Err(error) => {
            eprintln!("WASM execution error: {}", error);
            std::process::exit(1);
        }
    }
}
