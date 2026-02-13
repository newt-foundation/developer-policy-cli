// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {NewtonPolicyClient} from "@newton/contracts/src/mixins/NewtonPolicyClient.sol";
import {NewtonMessage} from "@newton/contracts/src/core/NewtonMessage.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {INewtonProverTaskManager} from "@newton/contracts/src/interfaces/INewtonProverTaskManager.sol";

/**
 * @title NewtonErc20
 * @notice An ERC20 token that extends NewtonPolicyClient, with policy-guarded mint and transfer functions
 */
contract NewtonErc20 is NewtonPolicyClient, ERC20 {
    error InvalidAttestation();
    error AttestationRequired();
    error InvalidIntentData();

    bytes4 private constant MINT_SELECTOR = bytes4(keccak256("mint(address,uint256)"));
    bytes4 private constant TRANSFER_SELECTOR = bytes4(keccak256("transfer(address,uint256)"));

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
     * @notice Transfers tokens to an address after validating the attestation.
     *         The recipient and amount are parsed from task.intent.data (must be ABI-encoded
     *         with function signature "transfer(address,uint256)") so they cannot be tampered with.
     * @param task The task generated to evaluate the intent
     * @param taskResponse The task response returned by the prover 
     * @param signatureData BLS signature data for verification
     */
    function transfer(
        INewtonProverTaskManager.Task calldata task,
        INewtonProverTaskManager.TaskResponse calldata taskResponse,
        bytes calldata signatureData
    ) external {
        require(_validateAttestationDirect(task, taskResponse, signatureData), InvalidAttestation());
        (address to, uint256 amount) = _decodeAddressAmountIntent(task.intent.data, TRANSFER_SELECTOR);
        _transfer(task.intent.from, to, amount);
    }

    /**
     * @notice Decodes (address, uint256) from intent data. Reverts if data is not ABI-encoded
     *         with the expected function selector (e.g. "mint(address,uint256)" or "transfer(address,uint256)").
     * @param data The ABI-encoded intent data (selector + address + uint256)
     * @param expectedSelector The required 4-byte function selector (MINT_SELECTOR or TRANSFER_SELECTOR)
     */
    function _decodeAddressAmountIntent(bytes calldata data, bytes4 expectedSelector) internal pure returns (address to, uint256 amount) {
        if (data.length < 4) revert InvalidIntentData();
        if (bytes4(data[0:4]) != expectedSelector) revert InvalidIntentData();
        if (data.length != 4 + 64) revert InvalidIntentData(); // selector + 32 bytes address + 32 bytes uint256
        return abi.decode(data[4:], (address, uint256));
    }

    /**
     * @notice Mints tokens to an address after validating the attestation.
     *         The recipient and amount are parsed from task.intent.data (must be ABI-encoded
     *         with function signature "mint(address,uint256)") so they cannot be tampered with.
     * @param task The prover task to validate
     * @param taskResponse The task response
     * @param signatureData BLS signature data for verification
     */
    function mint(
        INewtonProverTaskManager.Task calldata task,
        INewtonProverTaskManager.TaskResponse calldata taskResponse,
        bytes calldata signatureData
    ) external {
        require(_validateAttestationDirect(task, taskResponse, signatureData), InvalidAttestation());
        (address to, uint256 amount) = _decodeAddressAmountIntent(task.intent.data, MINT_SELECTOR);
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


    function _validateAttestationDirect(
        INewtonProverTaskManager.Task calldata task,
        INewtonProverTaskManager.TaskResponse calldata taskResponse,
        bytes calldata signatureData
    ) internal returns (bool) {
        return INewtonProverTaskManager(_getNewtonPolicyTaskManager()).validateAttestationDirect(task, taskResponse, signatureData);
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

