// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
import {YourPolicyClient} from "./YourPolicyClient.sol";
import {INewtonPolicy} from "@newton/contracts/src/interfaces/INewtonPolicy.sol";

contract YourClientFactory {
    address public immutable CLIENT_IMPL;
    address public immutable TASK_MANAGER;

    constructor(
        address _impl,
        address _policyTaskManager
    ) {
        CLIENT_IMPL = _impl;
        TASK_MANAGER = _policyTaskManager;
    }

    function createClient(
        address policy,
        INewtonPolicy.PolicyConfig memory config
    ) external returns (YourPolicyClient client) {
        client = YourPolicyClient(Clones.clone(CLIENT_IMPL));
        client.initialize(TASK_MANAGER, policy, msg.sender);
        client.setParameters(config);
    }
}
