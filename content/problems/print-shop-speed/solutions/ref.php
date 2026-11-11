<?php
function minSpeed(array $pages, int $hours): int {
    $lo = 1;
    $hi = max($pages);
    while ($lo < $hi) {
        $mid = intdiv($lo + $hi, 2);
        $need = 0;
        foreach ($pages as $p) {
            $need += intdiv($p + $mid - 1, $mid);
        }
        if ($need <= $hours) {
            $hi = $mid;
        } else {
            $lo = $mid + 1;
        }
    }
    return $lo;
}
