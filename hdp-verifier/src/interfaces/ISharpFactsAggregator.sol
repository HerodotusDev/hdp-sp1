// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @notice Interface for the SharpFactsAggregator.
interface ISharpFactsAggregator {
    struct AggregatorState {
        bytes32 poseidonMmrRoot;
        bytes32 keccakMmrRoot;
        uint256 mmrSize;
        bytes32 continuableParentHash;
    }

    function aggregatorState() external view returns (AggregatorState memory);
}
