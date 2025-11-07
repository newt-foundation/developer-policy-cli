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

        // Parse individual fields from JSON
        bytes32 taskId = bytes32(attestationJson.parseRaw(".taskId"));
        bytes32 policyId = bytes32(attestationJson.parseRaw(".policyId"));
        address policyClient = attestationJson.readAddress(".policyClient");
        uint32 expiration = uint32(attestationJson.readUint(".expiration"));
        
        // Parse nested Intent struct
        address intentFrom = attestationJson.readAddress(".intent.from");
        address intentTo = attestationJson.readAddress(".intent.to");
        uint256 intentValue = attestationJson.readUint(".intent.value");
        bytes memory intentData = attestationJson.parseRaw(".intent.data");
        uint256 intentChainId = attestationJson.readUint(".intent.chainId");
        bytes memory intentFunctionSignature = attestationJson.parseRaw(".intent.functionSignature");
        
        NewtonMessage.Intent memory intent = NewtonMessage.Intent({
            from: intentFrom,
            to: intentTo,
            value: intentValue,
            data: intentData,
            chainId: intentChainId,
            functionSignature: intentFunctionSignature
        });
        
        // Construct the Attestation struct
        NewtonMessage.Attestation memory attestation = NewtonMessage.Attestation({
            taskId: taskId,
            policyId: policyId,
            policyClient: policyClient,
            intent: intent,
            expiration: expiration
        });

        // Call the swap function with the parsed attestation
        client.swap(attestation);

        vm.stopBroadcast();
    }
}
