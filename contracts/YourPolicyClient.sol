// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {NewtonPolicyClient} from "@newton/contracts/src/mixins/NewtonPolicyClient.sol";
import {INewtonPolicy} from "@newton/contracts/src/interfaces/INewtonPolicy.sol";
import {NewtonMessage} from "@newton/contracts/src/core/NewtonMessage.sol";

contract YourPolicyClient is NewtonPolicyClient {
	// Your contract's business logic goes here

    event Success();

    error InvalidAttestation();
	
    // since the factory is used to clone the client, the constructor doesn't need to do anything
    constructor() {}

    // this is called by the deploy script
    function initialize(
        address policyTaskManager,
        address policy, //refers to the policy template address
        address owner
    ) external {
        _initNewtonPolicyClient(policyTaskManager, policy, owner);
    }
	
    // this function duplicates the functionality the base NewtonPolicyClient without permissioning
    // don't do this in production
	function setParameters(INewtonPolicy.PolicyConfig memory _config) external {
        _setPolicy(_config);
    }
  
    // this is the policy guarded function
    function swap(NewtonMessage.Attestation memory attestation) external {
        require(_validateAttestation(attestation), InvalidAttestation());

        // Your function's business logic goes here

        emit Success();
    }
}
