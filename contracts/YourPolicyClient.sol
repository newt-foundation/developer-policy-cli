// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {NewtonPolicyClient} from "@newton/contracts/mixins/NewtonPolicyClient.sol";
import {INewtonPolicy} from "@newton/contracts/interfaces/INewtonPolicy.sol";
import {NewtonMessage} from "@newton/contracts/core/NewtonMessage.sol";

contract YourPolicyClient is NewtonPolicyClient {
	// Your contract's business logic goes here

    event Success();

    error InvalidAttestation();
	
	// You may intialize NewtonPolicyClient in the constructor
	// or use your own initialize method
    constructor(
        address policyTaskManager,
        address policy, //refers to the policy template address
        address policyClientOwner //defaults to the deployer 
    ) {
        _initNewtonPolicyClient(policyTaskManager, policy, policyClientOwner);
    }
	
	function setParameters(INewtonPolicy.PolicyConfig memory _config) external {
        _setPolicy(_config);
    }
  
    function swap(NewtonMessage.Attestation memory attestation) external {
        require(_validateAttestation(attestation), InvalidAttestation());

        // Your function's business logic goes here

        emit Success();
    }
}
