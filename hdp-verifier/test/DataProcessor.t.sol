// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {DataProcessor} from "../src/DataProcessor.sol";
import {IAggregatorsFactory} from "../src/interfaces/IAggregatorsFactory.sol";
import {ISharpFactsAggregator} from "../src/interfaces/ISharpFactsAggregator.sol";
import {SP1VerifierGateway} from "sp1-contracts/src/SP1VerifierGateway.sol";

contract MockAggregatorsFactory is IAggregatorsFactory {
    mapping(uint256 => ISharpFactsAggregator) public aggregatorsById;

    function createAggregator(uint256 id, ISharpFactsAggregator aggregator) external {
        aggregatorsById[id] = aggregator;
    }
}

contract MockSharpFactsAggregator is ISharpFactsAggregator {
    uint256 public usedMmrSize;
    bytes32 public usedMmrRoot;

    constructor(bytes32 keccakMmrRoot, uint256 mmrSize) {
        usedMmrRoot = keccakMmrRoot;
        usedMmrSize = mmrSize;
    }

    function aggregatorState() external view returns (AggregatorState memory) {
        return AggregatorState({
            poseidonMmrRoot: bytes32(0),
            keccakMmrRoot: usedMmrRoot,
            mmrSize: usedMmrSize,
            continuableParentHash: bytes32(0)
        });
    }
}

struct SP1ProofFixtureJson {
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
}

contract DataProcessorTest is Test {
    using stdJson for string;

    IAggregatorsFactory private aggregatorsFactory;
    ISharpFactsAggregator private sharpFactsAggregator;

    address verifier;
    DataProcessor public dataProcessor;

    function loadFixture() public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/src/fixtures/groth16-fixture.json");
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }

    function setUp() public {
        SP1ProofFixtureJson memory fixture = loadFixture();
        verifier = address(new SP1VerifierGateway(address(0x397A5f7f3dBd538f23DE225B51f532c34448dA9B)));

        // Factory for creating SHARP facts aggregators
        aggregatorsFactory = new MockAggregatorsFactory();

        dataProcessor = new DataProcessor(aggregatorsFactory, verifier, fixture.vkey);

        // Mock SHARP facts aggregator
        sharpFactsAggregator = new MockSharpFactsAggregator(
            bytes32(0x62d451ed3f131fa253957db4501b0f4b6eb3f29c706663be3f75a35b7b372a38), uint256(13024091)
        );

        // Create mock SHARP facts aggregator
        aggregatorsFactory.createAggregator(uint256(27), sharpFactsAggregator);
    }

    function test_ValidDataProcessorProof() public {
        vm.chainId(11155111);

        SP1ProofFixtureJson memory fixture = loadFixture();

        vm.mockCall(verifier, abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector), abi.encode(true));

        bytes memory res = dataProcessor.verifydataProcessorProof(fixture.publicValues, fixture.proof);

        console.logBytes(res);
    }

    function testFail_InvalidDataProcessorProof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        dataProcessor.verifydataProcessorProof(fixture.publicValues, fakeProof);
    }
}
