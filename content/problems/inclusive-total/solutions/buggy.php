<?php
function rangeTotal(array $nums, int $a, int $b): int {
    $total = 0;
    for ($i = $a; $i < $b; $i++) {
        $total += $nums[$i];
    }
    return $total;
}
