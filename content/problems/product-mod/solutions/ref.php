<?php
function productMod(array $nums): int {
    $mod = 1000000007;
    $acc = 1;
    foreach ($nums as $x) {
        // Both factors are below 2^30 after the modulo: the product fits in 64 bits.
        $acc = ($acc * ($x % $mod)) % $mod;
    }
    return $acc;
}
