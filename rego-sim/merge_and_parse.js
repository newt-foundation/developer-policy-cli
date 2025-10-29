// merge_and_parse.js
// Usage: node merge_and_parse.js policy_params_data.json wasmData.json output_data.json

const fs = require('fs');
const vm = require('vm');
const path = require('path');

// Load ethers.umd.min.js and expose ethers as a global
const ethersUMD = fs.readFileSync(path.join(__dirname, 'ethers.umd.min.js'), 'utf8');
const context = {
  atob: (str) => Buffer.from(str, 'base64').toString('binary'),
  btoa: (str) => Buffer.from(str, 'binary').toString('base64'),
  Buffer,
  console
};
vm.createContext(context);
vm.runInContext(ethersUMD, context);
const ethers = context.ethers;

if (process.argv.length < 5) {
  console.error('Usage: node merge_and_parse.js policy_params_data.json wasmData.json output_data.json');
  process.exit(1);
}

console.log(ethers.utils.getAddress('0x0000000000000000000000000000000000000000'));

const [paramsFile, wasmFile, outputFile] = process.argv.slice(2);

const params = JSON.parse(fs.readFileSync(paramsFile, 'utf8'));
const wasmData = JSON.parse(fs.readFileSync(wasmFile, 'utf8'));

const result = {
  params,
  data: wasmData
};

fs.writeFileSync(outputFile, JSON.stringify(result, null, 2));
console.log(`Wrote merged data to ${outputFile}`);
