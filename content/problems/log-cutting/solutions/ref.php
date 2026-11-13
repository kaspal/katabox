<?php
function cutCost(int $length, array $marks): int {
    sort($marks);
    $p = array_merge([0], $marks, [$length]);
    $m = count($p);
    $cost = array_fill(0, $m, array_fill(0, $m, 0));
    for ($gap = 2; $gap < $m; $gap++) {
        for ($i = 0; $i + $gap < $m; $i++) {
            $j = $i + $gap;
            $best = PHP_INT_MAX;
            for ($k = $i + 1; $k < $j; $k++) {
                $best = min($best, $cost[$i][$k] + $cost[$k][$j]);
            }
            $cost[$i][$j] = $p[$j] - $p[$i] + $best;
        }
    }
    return $cost[0][$m - 1];
}
