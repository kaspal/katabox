<?php
function freeNumbers(array $shelf, array $queries): array {
    $n = count($shelf);
    $lo = 0;
    $hi = $n - 1;
    while ($lo < $hi) {
        $mid = intdiv($lo + $hi, 2);
        if ($shelf[$mid] > $shelf[$hi]) {
            $lo = $mid + 1;
        } else {
            $hi = $mid;
        }
    }
    $p = $lo;
    $out = [];
    foreach ($queries as $k) {
        $a = 0;
        $b = $n;
        while ($a < $b) {
            $mid = intdiv($a + $b, 2);
            if ($shelf[($p + $mid) % $n] - ($mid + 1) >= $k) {
                $b = $mid;
            } else {
                $a = $mid + 1;
            }
        }
        $out[] = $k + $a;
    }
    return $out;
}
