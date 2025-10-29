## Using Regorus for Rego Policy Testing

This directory usesour included build of [regorus](https://github.com/microsoft/regorus) for evaluating Rego policies.

### Usage

To evaluate a policy with regorus:

```sh
./regorus eval --input input.json --data data.json --data policy.rego "data.example.allow"
```

Replace `input.json`, `data.json`, and `policy.rego` with your actual files as needed.