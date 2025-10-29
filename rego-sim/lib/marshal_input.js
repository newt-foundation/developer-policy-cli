// marshal_input.js
// Usage: node marshal_input.js test_intent.json input.json

const fs = require('fs');
const vm = require('vm');
const path = require('path');

// Load ethers.umd.min.js and expose ethers as a global
const ethersUMD = fs.readFileSync(path.join(__dirname, 'lib/ethers.umd.min.js'), 'utf8');
const context = {
  atob: (str) => Buffer.from(str, 'base64').toString('binary'),
  btoa: (str) => Buffer.from(str, 'binary').toString('base64'),
  Buffer,
  console
};
vm.createContext(context);
vm.runInContext(ethersUMD, context);

if (process.argv.length < 4) {
  console.error('Usage: node merge_and_parse.js test_intent.json input.json');
  process.exit(1);
}

const [intentFile, outputFile] = process.argv.slice(2);
const intent = JSON.parse(fs.readFileSync(intentFile, 'utf8'));

// Convert camelCase to snake_case for keys
function toSnakeCase(str) {
  return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

const output = {};
for (const key in intent) {
  if (Object.hasOwnProperty.call(intent, key)) {
    output[toSnakeCase(key)] = intent[key];
  }
}


// Decode function signature (hex to ascii)
let decodedFunctionSignature = '';
if (output.function_signature && output.function_signature.startsWith('0x')) {
  const hex = output.function_signature.slice(2);
  decodedFunctionSignature = Buffer.from(hex, 'hex').toString('utf8');
  output.decoded_function_signature = decodedFunctionSignature;
}

// Parse function object from signature
let functionObj = null;
try {
  if (decodedFunctionSignature) {
    functionObj = context.ethers.utils.FunctionFragment.from(decodedFunctionSignature);
    // Add default param names if missing
    functionObj = JSON.parse(JSON.stringify(functionObj));
    functionObj.inputs = functionObj.inputs.map((input, idx) => ({
      type: input.type,
      name: input.name || `param${idx}`
    }));
    output.function = functionObj;
  } else {
    output.function = {
      name: '',
      type: 'function',
      stateMutability: '',
      inputs: [],
      outputs: []
    };
  }
} catch (e) {
  output.function = {
    name: '',
    type: 'function',
    stateMutability: '',
    inputs: [],
    outputs: []
  };
}

// Decode calldata arguments
try {
  if (functionObj && output.data) {
    let ifaceSignature = decodedFunctionSignature.trim();
    if (!ifaceSignature.startsWith('function')) {
      ifaceSignature = 'function ' + ifaceSignature;
    }
    const iface = new context.ethers.utils.Interface([ifaceSignature]);
    const decodedArgs = iface.decodeFunctionData(functionObj.name, output.data);
    // Convert BigNumber and address types to strings for JSON output
    output.decoded_function_arguments = Array.from(decodedArgs).map((arg, idx) => {
      if (functionObj.inputs[idx].type === 'address') {
        return context.ethers.utils.getAddress(arg);
      }
      if (arg && arg._isBigNumber) {
        return arg.toString();
      }
      return arg;
    });
  } else {
    output.decoded_function_arguments = [];
  }
} catch (e) {
  output.decoded_function_arguments = [];
}

fs.writeFileSync(outputFile, JSON.stringify(output, null, 2));
console.log(`Wrote parsed intent to ${outputFile}`);
