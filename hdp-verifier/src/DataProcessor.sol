// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "sp1-contracts/src/ISP1Verifier.sol";
import {IAggregatorsFactory} from "./interfaces/IAggregatorsFactory.sol";
import {ISharpFactsAggregator} from "./interfaces/ISharpFactsAggregator.sol";

struct PublicValuesStruct {
    /// @dev The id of the MMR.
    uint256 mmrId;
    /// @dev The size of the MMR.
    uint256 mmrSize;
    /// @dev The root of the MMR.
    bytes32 mmrRoot;
    /// @dev result of program
    bytes result;
}

/// MMR doesn't exist.
error InvalidMMR();

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

    /// @notice interface to the aggregators factory
    IAggregatorsFactory public AGGREGATORS_FACTORY;

    /// @notice mapping of  mmr id => mmr size => mmr root
    mapping(uint256 => mapping(uint256 => bytes32)) public cachedMMRsRoots;

    /// @notice emitted when a new MMR root is cached
    event MmrRootCached(uint256 mmrId, uint256 mmrSize, bytes32 mmrRoot);

    constructor(IAggregatorsFactory aggregatorsFactory, address _verifier, bytes32 _dataProcessorProgramVKey) {
        AGGREGATORS_FACTORY = aggregatorsFactory;
        verifier = _verifier;
        dataProcessorProgramVKey = _dataProcessorProgramVKey;
    }

    /// @notice Caches the MMR root for a given MMR id
    /// @notice Get MMR size and root from the aggregator and cache it
    function cacheMmrRoot(uint256 mmrId) public {
        ISharpFactsAggregator aggregator = AGGREGATORS_FACTORY.aggregatorsById(mmrId);
        ISharpFactsAggregator.AggregatorState memory aggregatorState = aggregator.aggregatorState();
        cachedMMRsRoots[mmrId][aggregatorState.mmrSize] = aggregatorState.poseidonMmrRoot;

        emit MmrRootCached(mmrId, aggregatorState.mmrSize, aggregatorState.poseidonMmrRoot);
    }

    /// @notice The entrypoint for verifying the proof of a dataProcessor number.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifydataProcessorProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (bytes memory)
    {
        ISP1Verifier(verifier).verifyProof(dataProcessorProgramVKey, _publicValues, _proofBytes);
        PublicValuesStruct memory publicValues = abi.decode(_publicValues, (PublicValuesStruct));

        ISharpFactsAggregator aggregator = AGGREGATORS_FACTORY.aggregatorsById(publicValues.mmrId);
        ISharpFactsAggregator.AggregatorState memory aggregatorState = aggregator.aggregatorState();

        if (publicValues.mmrRoot != aggregatorState.keccakMmrRoot) {
            revert InvalidMMR();
        }

        return (publicValues.result);
    }
}
