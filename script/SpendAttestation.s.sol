// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {NewtonMessage} from "@newton/contracts/src/core/NewtonMessage.sol";

import {YourPolicyClient} from "../contracts/YourPolicyClient.sol";

contract ClientAttestationSpender is Script {
    using stdJson for *;

    address internal _deployer;
    address internal _policyClient;

    error AttestationFileDoesNotExist();

    function setUp() public virtual {
        _deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        _policyClient = vm.envAddress("POLICY_CLIENT");
    }

    function run() external {
        vm.startBroadcast(_deployer);
        YourPolicyClient client = YourPolicyClient(_policyClient);

        string memory attestationJson;
        string memory paramsFilePath = vm.envOr("ATTESTATION_FILE", string(""));
        if (bytes(paramsFilePath).length > 0) {
            require(vm.exists(paramsFilePath), AttestationFileDoesNotExist());
            attestationJson = vm.readFile(paramsFilePath);
        } else {
            revert("ATTESTATION_FILE must be set");
        }

        // Parse the JSON into the Attestation struct
        bytes memory attestationData = attestationJson.parseRaw("$");
        NewtonMessage.Attestation memory attestation = abi.decode(attestationData, (NewtonMessage.Attestation));

        // Call the swap function with the parsed attestation
        client.swap(attestation);

        vm.stopBroadcast();
    }
}
