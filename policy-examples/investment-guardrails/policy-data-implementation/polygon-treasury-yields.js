import { fetch as httpFetch } from 'newton:provider/http@0.1.0';

export function run(date) {
  const response = httpFetch({
    url: `https://o66wu5mr47.execute-api.us-east-2.amazonaws.com/default/polygon/treasury-yields${date ? '?date=' + date : ''}`,
    method: "GET",
    headers: [],
    body: null
  });
  
  const body = JSON.parse(new TextDecoder().decode(new Uint8Array(response.body)));
  
  return JSON.stringify(body.result[0]);
}
