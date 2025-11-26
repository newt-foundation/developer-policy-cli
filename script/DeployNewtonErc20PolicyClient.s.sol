// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";

import {NewtonErc20} from "../contracts/NewtonErc20.sol";

contract DeployNewtonErc20PolicyClient is Script {
    using stdJson for *;

    address internal _deployer;
    address internal _policy;

    error DeploymentFileDoesNotExist();

    function setUp() public virtual {
        _deployer = vm.rememberKey(vm.envUint("PRIVATE_KEY"));
        _policy = vm.envAddress("POLICY");
    }

    function run() external returns (NewtonErc20 token) {
        vm.startBroadcast(_deployer);

        string memory env = vm.envOr("DEPLOYMENT_ENV", string("prod"));

        string memory fileName = string.concat("lib/newton-contracts/script/deployments/newton-prover/", vm.toString(block.chainid), "-", env, ".json");
        require(vm.exists(fileName), DeploymentFileDoesNotExist());

        string memory json = vm.readFile(fileName);
        address newtonProverTaskManager = json.readAddress(".addresses.newtonProverTaskManager");

        // Get token name and symbol from environment variables, with defaults
        string memory tokenName = vm.envOr("TOKEN_NAME", string("Newton ERC20 Token"));
        string memory tokenSymbol = vm.envOr("TOKEN_SYMBOL", string("NEWT"));

        token = new NewtonErc20(tokenName, tokenSymbol);

        token.initialize(newtonProverTaskManager, _policy, msg.sender);

        vm.stopBroadcast();
    }
}

