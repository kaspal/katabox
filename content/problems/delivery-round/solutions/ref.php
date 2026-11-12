<?php
function shortestRound(array $dist): int {
    $n = count($dist);
    if ($n === 1) {
        return 0;
    }
    $full = (1 << $n) - 1;
    $inf = PHP_INT_MAX;
    $best = array_fill(0, ($full + 1) * $n, $inf);
    for ($last = 0; $last < $n; $last++) {
        $best[$full * $n + $last] = $dist[$last][0];
    }
    // Only masks containing the depot (odd masks), from the fullest down.
    for ($mask = $full - 2; $mask >= 1; $mask -= 2) {
        for ($last = 0; $last < $n; $last++) {
            if ((($mask >> $last) & 1) === 0) {
                continue;
            }
            $b = $inf;
            $row = $dist[$last];
            for ($j = 1; $j < $n; $j++) {
                if (($mask >> $j) & 1) {
                    continue;
                }
                $v = $row[$j] + $best[($mask | (1 << $j)) * $n + $j];
                if ($v < $b) {
                    $b = $v;
                }
            }
            $best[$mask * $n + $last] = $b;
        }
    }
    return $best[$n];
}
