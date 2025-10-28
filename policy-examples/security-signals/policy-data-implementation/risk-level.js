import { fetch as httpFetch } from 'newton:provider/http@0.1.0';

export function run({ address }) {
  const response = httpFetch({
    url: `https://o66wu5mr47.execute-api.us-east-2.amazonaws.com/default/risk/address/${address}`,
    method: "GET",
    headers: [],
    body: null
  });

  const body = JSON.parse(new TextDecoder().decode(new Uint8Array(response.body)));

  // Extract wallet matches and compute highest risk level
  const walletMatches = body.data?.wallet_matches || [];
  const riskLevels = walletMatches.map(w => w.risk_level);
  const rank = { low: 1, medium: 2, high: 3, critical: 4 };
  const highestRiskLevel = riskLevels.length
    ? riskLevels.reduce((a, b) => (rank[a] > rank[b] ? a : b))
    : "low";

  return JSON.stringify({
    status: response.status,
    address,
    risk_level: highestRiskLevel,
    wallet_matches: walletMatches,
  });
}
