<?php
function tilings(int $n): int {
    $mod = 1000000007;
    [$a, $b, $c] = [0, 0, 1];
    for ($i = 0; $i < $n; $i++) {
        [$a, $b, $c] = [$b, $c, ($a + $b + $c) % $mod];
    }
    return $c;
}
