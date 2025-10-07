// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";

import {YourClientFactory} from "../contracts/YourClientFactory.sol";
import {YourPolicyClient} from "../contracts/YourPolicyClient.sol";

contract ClientFactoryDeployer is Script {
    using stdJson for *;

    address internal _deployer;
    address internal _policy;

    error DeploymentFileDoesNotExist();

    function setUp() public virtual {
        _deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
    }

    function run() external returns (YourPolicyClient clientImplementation, YourClientFactory factory) {
        vm.startBroadcast(_deployer);

        string memory fileName = string.concat("lib/newton-contracts/script/deployments/newton-prover/", vm.toString(block.chainid), ".json");
        require(vm.exists(fileName), DeploymentFileDoesNotExist());

        string memory json = vm.readFile(fileName);
        address newtonProverTaskManager = json.readAddress(".addresses.newtonProverTaskManager");

        clientImplementation = new YourPolicyClient();

        factory = new YourClientFactory(address(clientImplementation), newtonProverTaskManager);

        vm.stopBroadcast();
    }
}
