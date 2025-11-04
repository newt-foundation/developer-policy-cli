// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";

import {YourPolicyClient} from "../contracts/YourPolicyClient.sol";

contract ClientDeployer is Script {
    using stdJson for *;

    address internal _deployer;
    address internal _policy;

    error DeploymentFileDoesNotExist();

    function setUp() public virtual {
        _deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        _policy = vm.envAddress("POLICY");
    }

    function run() external returns (YourPolicyClient client) {
        vm.startBroadcast(_deployer);

        string memory env = vm.envOr("DEPLOYMENT_ENV", string("prod"));

        string memory fileName = string.concat("lib/newton-contracts/script/deployments/newton-prover/", vm.toString(block.chainid), "-", env, ".json");
        require(vm.exists(fileName), DeploymentFileDoesNotExist());

        string memory json = vm.readFile(fileName);
        address newtonProverTaskManager = json.readAddress(".addresses.newtonProverTaskManager");

        client = new YourPolicyClient();

        client.initialize(newtonProverTaskManager, _policy, msg.sender);

        vm.stopBroadcast();
    }
}
