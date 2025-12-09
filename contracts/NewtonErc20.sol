// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {NewtonPolicyClient} from "@newton/contracts/src/mixins/NewtonPolicyClient.sol";
import {NewtonMessage} from "@newton/contracts/src/core/NewtonMessage.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title NewtonErc20
 * @notice An ERC20 token that extends NewtonPolicyClient, with policy-guarded mint and transfer functions
 */
contract NewtonErc20 is NewtonPolicyClient, ERC20 {
    error InvalidAttestation();
    error AttestationRequired();

    /**
     * @notice Constructor for NewtonErc20
     * @param name The name of the token
     * @param symbol The symbol of the token
     */
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}

    /**
     * @notice Initializes the policy client
     * @param policyTaskManager The address of the policy task manager
     * @param policy The address of the policy template
     * @param owner The owner of the policy client
     */
    function initialize(
        address policyTaskManager,
        address policy, //refers to the policy template address
        address owner
    ) external {
        _initNewtonPolicyClient(policyTaskManager, policy, owner);
    }

    /**
     * @notice Mints tokens to an address after validating the attestation
     * @param to The address to mint tokens to
     * @param amount The amount of tokens to mint
     * @param attestation The attestation to validate
     */
    function mint(
        address to,
        uint256 amount,
        NewtonMessage.Attestation calldata attestation
    ) external {
        require(_validateAttestation(attestation), InvalidAttestation());
        _mint(to, amount);
    }

    /**
     * @notice Override standard ERC20 transfer to require attestation
     * @dev This function is disabled - use transfer(address,uint256,Attestation) instead
     * @return success Always reverts - attestation required
     */
    function transfer(
        address /* to */,
        uint256 /* amount */
    ) public virtual override returns (bool) {
        revert AttestationRequired();
    }

    /**
     * @notice Transfers tokens after validating the attestation
     * @param to The address to transfer tokens to
     * @param amount The amount of tokens to transfer
     * @param attestation The attestation to validate
     * @return success Whether the transfer was successful
     */
    function transfer(
        address to,
        uint256 amount,
        NewtonMessage.Attestation calldata attestation
    ) external returns (bool) {
        require(_validateAttestation(attestation), InvalidAttestation());
        _transfer(msg.sender, to, amount);
        return true;
    }
}

