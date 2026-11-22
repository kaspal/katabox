<?php
function twoPivots(array $weights): int {
    $n = count($weights);
    $prefix = array_fill(0, $n + 1, 0);
    for ($k = 0; $k < $n; $k++) {
        $prefix[$k + 1] = $prefix[$k] + $weights[$k];
    }
    $total = $prefix[$n];
    $seen = [];
    $count = 0;
    for ($j = 1; $j < $n; $j++) {
        $i = $j - 1;
        $key = $prefix[$i] . ',' . $weights[$i];
        $seen[$key] = ($seen[$key] ?? 0) + 1;
        $right = $total - $prefix[$j + 1];
        $count += $seen[$right . ',' . ($prefix[$j] - 2 * $right)] ?? 0;
    }
    return $count;
}
