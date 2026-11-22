<?php
function firstAtLeast(array $sorted, int $x): int {
    $lo = 0;
    $hi = count($sorted) - 1;
    while ($lo < $hi) {
        $mid = intdiv($lo + $hi, 2);
        if ($sorted[$mid] < $x) {
            $lo = $mid + 1;
        } else {
            $hi = $mid;
        }
    }
    return $lo;
}
