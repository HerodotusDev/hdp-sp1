// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {DataProcessor} from "../src/DataProcessor.sol";
import {IAggregatorsFactory} from "../src/interfaces/IAggregatorsFactory.sol";
import {SP1VerifierGateway} from "sp1-contracts/src/SP1VerifierGateway.sol";

struct SP1ProofFixtureJson {
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
}

contract DataProcessorTest is Test {
    using stdJson for string;

    address verifier;
    DataProcessor public dataProcessor;

    function loadFixture() public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(
            root,
            "/src/fixtures/groth16-fixture.json"
        );
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }

    function setUp() public {
        SP1ProofFixtureJson memory fixture = loadFixture();
        verifier = address(new SP1VerifierGateway(address(1)));
        IAggregatorsFactory aggregatorsFactory = IAggregatorsFactory(
            0x3CFf12e2F0301acA56527BdA2e86DC7d97eBC903
        );
        dataProcessor = new DataProcessor(
            aggregatorsFactory,
            verifier,
            fixture.vkey
        );
    }

    function test_ValidDataProcessorProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        vm.mockCall(
            verifier,
            abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector),
            abi.encode(true)
        );

        bytes32 root = dataProcessor.verifydataProcessorProof(
            fixture.publicValues,
            fixture.proof
        );

        console.logBytes32(root);
    }

    function testFail_InvalidDataProcessorProof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        dataProcessor.verifydataProcessorProof(fixture.publicValues, fakeProof);
    }
}
