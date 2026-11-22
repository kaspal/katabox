<?php
function bestRun(array $profits, int $k): int {
    $n = count($profits);
    $prefix = array_fill(0, $n + 1, 0);
    for ($i = 0; $i < $n; $i++) {
        $prefix[$i + 1] = $prefix[$i] + $profits[$i];
    }
    $lows = [];
    $head = 0;
    $tail = 0;
    $best = PHP_INT_MIN;
    for ($j = 1; $j <= $n; $j++) {
        $i = $j - 1;
        while ($tail > $head && $prefix[$lows[$tail - 1]] >= $prefix[$i]) {
            $tail--;
        }
        $lows[$tail++] = $i;
        if ($lows[$head] < $j - $k) {
            $head++;
        }
        $best = max($best, $prefix[$j] - $prefix[$lows[$head]]);
    }
    return $best;
}
