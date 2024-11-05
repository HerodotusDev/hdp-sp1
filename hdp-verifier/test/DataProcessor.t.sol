// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {DataProcessor} from "../src/DataProcessor.sol";
import {SP1VerifierGateway} from "sp1-contracts/src/SP1VerifierGateway.sol";

struct SP1ProofFixtureJson {
    uint32 a;
    uint32 b;
    uint32 n;
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
        string memory path = string.concat(root, "/src/fixtures/fixture.json");
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }

    function setUp() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        verifier = address(new SP1VerifierGateway(address(1)));
        dataProcessor = new DataProcessor(verifier, fixture.vkey);
    }

    function test_ValidFibonacciProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        vm.mockCall(
            verifier,
            abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector),
            abi.encode(true)
        );

        (uint32 n, uint32 a, uint32 b) = dataProcessor.verifyFibonacciProof(
            fixture.publicValues,
            fixture.proof
        );
        assert(n == fixture.n);
        assert(a == fixture.a);
        assert(b == fixture.b);
    }

    function testFail_InvalidFibonacciProof() public view {
        SP1ProofFixtureJson memory fixture = loadFixture();

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        dataProcessor.verifyFibonacciProof(fixture.publicValues, fakeProof);
    }
}
