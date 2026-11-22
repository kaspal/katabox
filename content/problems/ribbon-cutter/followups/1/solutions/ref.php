<?php
function longestPieces(array $ribbons, array $orders): array {
    $m = max($ribbons);
    $n = count($ribbons);
    $atMost = array_fill(0, $m + 1, 0);
    foreach ($ribbons as $r) {
        $atMost[$r]++;
    }
    for ($v = 1; $v <= $m; $v++) {
        $atMost[$v] += $atMost[$v - 1];
    }
    $count = array_fill(0, $m + 1, 0);
    for ($len = 1; $len <= $m; $len++) {
        $total = 0;
        for ($k = $len; $k <= $m; $k += $len) {
            $total += $n - $atMost[$k - 1];
        }
        $count[$len] = $total;
    }
    $out = [];
    foreach ($orders as $need) {
        $lo = 0;
        $hi = $m;
        while ($lo < $hi) {
            $mid = intdiv($lo + $hi + 1, 2);
            if ($count[$mid] >= $need) {
                $lo = $mid;
            } else {
                $hi = $mid - 1;
            }
        }
        $out[] = $lo;
    }
    return $out;
}
