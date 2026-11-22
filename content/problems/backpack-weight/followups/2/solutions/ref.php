<?php
function bestPackHuge(array $weights, array $values, int $capacity): int {
    $total = array_sum($values);
    $inf = PHP_INT_MAX;
    $lightest = array_fill(0, $total + 1, $inf);
    $lightest[0] = 0;
    foreach ($weights as $i => $w) {
        $v = $values[$i];
        for ($t = $total; $t >= $v; $t--) {
            if ($lightest[$t - $v] !== $inf && $lightest[$t - $v] + $w < $lightest[$t]) {
                $lightest[$t] = $lightest[$t - $v] + $w;
            }
        }
    }
    for ($t = $total; $t >= 0; $t--) {
        if ($lightest[$t] <= $capacity) {
            return $t;
        }
    }
    return 0;
}
