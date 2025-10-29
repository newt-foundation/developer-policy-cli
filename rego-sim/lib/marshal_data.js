// marshal_data.js
// Usage: node marshal_data.js policy_params_data.json wasmData.json output_data.json

const fs = require('fs');

if (process.argv.length < 5) {
  console.error('Usage: node marshal_data.js policy_params_data.json wasmData.json output_data.json');
  process.exit(1);
}

const [paramsFile, wasmFile, outputFile] = process.argv.slice(2);

const params = JSON.parse(fs.readFileSync(paramsFile, 'utf8'));
const wasmData = JSON.parse(fs.readFileSync(wasmFile, 'utf8'));

const result = {
  params,
  data: wasmData
};

fs.writeFileSync(outputFile, JSON.stringify(result, null, 2));
console.log(`Wrote marshaled data to ${outputFile}`);
