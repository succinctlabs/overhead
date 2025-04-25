// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

contract MyFib {
    function fib(uint128 n) external pure returns (uint128 b) {
        if (n == 0) {
            return 0;
        }
        uint128 a = 1;
        b = 1;
        for (uint128 i = 2; i < n; i++) {
            uint128 c = (a + b) % 170141183460469231731687303715884105727;
            a = b;
            b = c;
        }
        return b;
    }
}
