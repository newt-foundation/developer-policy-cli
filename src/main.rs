mod bindings;
use bindings::wasi::http::types::*;
use bindings::wasi::http::outgoing_handler;
use bindings::wasi::io::poll;

fn main() {
    let request = OutgoingRequest::new(Fields::new());
    request.set_method(&Method::Get).unwrap();
    request.set_scheme(Some(&Scheme::Https)).unwrap();
    request.set_authority(Some("httpbin.org")).unwrap();
    request.set_path_with_query(Some("/get")).unwrap();
    
    let future = outgoing_handler::handle(request, Some(RequestOptions::new())).unwrap();
    poll::poll(&[&future.subscribe()]);
    
    let response = future.get().unwrap().unwrap().unwrap();
    let body = response.consume().unwrap();
    let stream = body.stream().unwrap();
    
    let mut data = Vec::new();
    loop {
        match stream.read(4096) {
            Ok(chunk) if !chunk.is_empty() => data.extend(chunk),
            _ => break,
        }
    }
    
    println!("{}", String::from_utf8(data).unwrap());
}
