import { fetch as httpFetch } from 'newton:provider/http@0.1.0';

const RPC_URLS = {
  '1': "https://eth.drpc.org",
  '11155111': "https://sepolia.drpc.org",
};

async function getGasPrice(chainId) {

  const rpcUrl = RPC_URLS[chainId];
  if (typeof rpcUrl !== "string") throw new Error(`Unsupported chainId: ${chainId}`);

  const body = {
    jsonrpc: "2.0",
    id: 1,
    method: "eth_gasPrice",
    params: []
  };

  const request = {
    url: rpcUrl,
    method: "POST",
    headers: [
      ["Content-Type", "application/json"]
    ],
    body: new TextEncoder().encode(JSON.stringify(body))
  };

  const res = await httpFetch(request);

  const decoder = new TextDecoder();
  const jsonStr = decoder.decode(res.body);
  const json = JSON.parse(jsonStr);

  const weiHex = json.result;
  const wei = parseInt(weiHex, 16);
  const gwei = wei / 1e9;

  return gwei;
}
export async function run() {
  try {
    const mainnetGasPrice = await getGasPrice('1');
    const sepoliaGasPrice = await getGasPrice('11155111');
    const output = {
      '1': mainnetGasPrice,
      '11155111': sepoliaGasPrice
    };
    return JSON.stringify(output);
  } catch (err) {
    return JSON.stringify({ error: err.message || String(err) });
  }
};
