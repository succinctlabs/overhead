// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

contract MyFib {
    function fib(uint64 n) external pure returns(uint64 b) { 
        if (n == 0) {
            return 0;   
        }
        uint64 a = 1;
        b = 1;
        for (uint64 i = 2; i < n; i++) {
            uint64 c = (a + b) % 65776547668456965;
            a = b;
            b = c;
        }
        return b;
    }
}