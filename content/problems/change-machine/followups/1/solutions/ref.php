<?php
function countWays(array $coins, int $amount): int {
    $ways = array_fill(0, $amount + 1, 0);
    $ways[0] = 1;
    foreach ($coins as $c) {
        for ($a = $c; $a <= $amount; $a++) {
            $ways[$a] = ($ways[$a] + $ways[$a - $c]) % 1000000007;
        }
    }
    return $ways[$amount];
}
