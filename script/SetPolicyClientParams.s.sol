// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {INewtonPolicy} from "@newton/contracts/src/interfaces/INewtonPolicy.sol";

import {YourPolicyClient} from "../contracts/YourPolicyClient.sol";

contract PolicyClientParamsSetter is Script {
    using stdJson for *;

    address internal _deployer;
    address internal _policyClient;

    error DeploymentFileDoesNotExist();

    function setUp() public virtual {
        _deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        _policyClient = vm.envAddress("POLICY_CLIENT");
    }

    function run() external {
        vm.startBroadcast(_deployer);

        // Attach to the already deployed client contract
        YourPolicyClient client = YourPolicyClient(_policyClient);

        string memory policyParamsJson = vm.envString("POLICY_PARAMS");
        uint32 expireAfter = uint32(vm.envUint("EXPIRE_AFTER"));
        bytes memory policyParams = bytes(policyParamsJson);

        INewtonPolicy.PolicyConfig memory config = INewtonPolicy.PolicyConfig({
            policyParams: policyParams,
            expireAfter: expireAfter
        });

        client.setPolicy(config);

        vm.stopBroadcast();
    }
}
