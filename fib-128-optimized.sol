// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.25;

contract MyFib {
    uint256 private constant MOD = 170141183460469231731687303715884105727;
    
    function fib(uint256 n) external pure returns (uint128) {
        if (n == 0) return 0;
        unchecked {
            uint256 a = 1;
            uint256 b = 1;
            for (uint256 i = 2; i < n; ++i) {
                uint256 c = addmod(a, b, MOD);
                a = b;
                b = c;
            }
            return uint128(b);
        }
    }
}