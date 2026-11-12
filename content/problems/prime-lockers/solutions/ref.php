<?php
function primeCounts(array $limits): array {
    $m = max($limits);
    // One byte per number: "1" prime, "0" not.
    $sieve = str_repeat('1', $m + 1);
    $sieve[0] = '0';
    $sieve[1] = '0';
    for ($p = 2; $p * $p <= $m; $p++) {
        if ($sieve[$p] === '0') {
            continue;
        }
        for ($q = $p * $p; $q <= $m; $q += $p) {
            $sieve[$q] = '0';
        }
    }
    $count = array_fill(0, $m + 1, 0);
    $running = 0;
    for ($x = 0; $x <= $m; $x++) {
        if ($sieve[$x] === '1') {
            $running++;
        }
        $count[$x] = $running;
    }
    $out = [];
    foreach ($limits as $x) {
        $out[] = $count[$x];
    }
    return $out;
}
