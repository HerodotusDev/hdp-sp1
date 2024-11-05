// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "sp1-contracts/src/ISP1Verifier.sol";

/// @title DataProcessor.
/// @author Succinct Labs
/// @notice This contract implements a simple example of verifying the proof of a computing a
///         fibonacci number.
contract DataProcessor {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://docs.succinct.xyz/onchain-verification/contract-addresses
    address public verifier;

    /// @notice The verification key for the fibonacci program.
    bytes32 public fibonacciProgramVKey;

    constructor(address _verifier, bytes32 _fibonacciProgramVKey) {
        verifier = _verifier;
        fibonacciProgramVKey = _fibonacciProgramVKey;
    }

    /// @notice The entrypoint for verifying the proof of a fibonacci number.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifyFibonacciProof(
        bytes calldata _publicValues,
        bytes calldata _proofBytes
    ) public view returns (uint32, uint32, uint32) {
        ISP1Verifier(verifier).verifyProof(
            fibonacciProgramVKey,
            _publicValues,
            _proofBytes
        );
        (uint32 n, uint32 a, uint32 b) = abi.decode(
            _publicValues,
            (uint32, uint32, uint32)
        );
        return (n, a, b);
    }
}
