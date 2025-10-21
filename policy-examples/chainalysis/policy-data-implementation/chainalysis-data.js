import { fetch as httpFetch } from 'newton:provider/http@0.1.0';

export function run(address) {
  const response = httpFetch({
    url: `https://o66wu5mr47.execute-api.us-east-2.amazonaws.com/default/chainalysis/address/${address}`,
    method: "GET",
    headers: [],
    body: null
  });
  
  const body = JSON.parse(new TextDecoder().decode(new Uint8Array(response.body)));
  
  return JSON.stringify({ 
    status: response.status,
    address,
    sanctioned: body.identifications.length > 0,
    identifications: body.identifications,
  });
}