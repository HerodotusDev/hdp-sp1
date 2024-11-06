// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "sp1-contracts/src/ISP1Verifier.sol";

/// @title DataProcessor.
/// @author Herodotus Dev Ltd.
/// @notice Verifies proof of data processor program that verifies MMR, MPT proof.
contract DataProcessor {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://docs.succinct.xyz/onchain-verification/contract-addresses
    address public verifier;

    /// @notice The verification key for the dataProcessor program.
    bytes32 public dataProcessorProgramVKey;

    constructor(address _verifier, bytes32 _dataProcessorProgramVKey) {
        verifier = _verifier;
        dataProcessorProgramVKey = _dataProcessorProgramVKey;
    }

    /// @notice The entrypoint for verifying the proof of a dataProcessor number.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifydataProcessorProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (uint256)
    {
        ISP1Verifier(verifier).verifyProof(dataProcessorProgramVKey, _publicValues, _proofBytes);
        uint256 v = abi.decode(_publicValues, (uint256));
        return (v);
    }
}
