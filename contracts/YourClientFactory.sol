// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
import {YourPolicyClient} from "./YourPolicyClient.sol";
import {INewtonPolicy} from "@newton/contracts/interfaces/INewtonPolicy.sol";

contract YourClientFactory {
    address public immutable CLIENT_IMPL;

    constructor(
        address _impl
    ) {
        CLIENT_IMPL = _impl;
    }

    function createClient(
        INewtonPolicy.PolicyConfig memory config
    ) external returns (YourPolicyClient client) {
        client = YourPolicyClient(Clones.clone(CLIENT_IMPL));
        client.setParameters(config);
    }
}
