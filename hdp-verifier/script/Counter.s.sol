// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {DataProcessor} from "../src/DataProcessor.sol";

contract DataProcessorScript is Script {
    DataProcessor public dp;

    function setUp() public {}

    // function run() public {
    //     vm.startBroadcast();

    //     dp = new DataProcessor();

    //     vm.stopBroadcast();
    // }
}
