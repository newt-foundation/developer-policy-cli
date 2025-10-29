// merge_and_parse.js
// Usage: node merge_and_parse.js policy_params_data.json wasmData.json output_data.json

const fs = require('fs');
// const { ethers } = require('ethers');

if (process.argv.length < 5) {
  console.error('Usage: node merge_and_parse.js policy_params_data.json wasmData.json output_data.json');
  process.exit(1);
}

const [paramsFile, wasmFile, outputFile] = process.argv.slice(2);

const params = JSON.parse(fs.readFileSync(paramsFile, 'utf8'));
const wasmData = JSON.parse(fs.readFileSync(wasmFile, 'utf8'));

// Example: parse a function call from params or elsewhere (customize as needed)
// const iface = new ethers.utils.Interface(abi);
// const parsed = iface.parseTransaction({ data: params.data });
// params.function = parsed;

const result = {
  params,
  data: wasmData
};

fs.writeFileSync(outputFile, JSON.stringify(result, null, 2));
console.log(`Wrote merged data to ${outputFile}`);
