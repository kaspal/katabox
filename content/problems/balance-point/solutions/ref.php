<?php
function balancePoint(array $weights): int {
    $total = array_sum($weights);
    $left = 0;
    foreach ($weights as $i => $w) {
        if ($left === $total - $left - $w) {
            return $i;
        }
        $left += $w;
    }
    return -1;
}
