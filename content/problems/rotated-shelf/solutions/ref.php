<?php
function findLabel(array $shelf, int $label): int {
    $lo = 0;
    $hi = count($shelf) - 1;
    while ($lo <= $hi) {
        $m = intdiv($lo + $hi, 2);
        if ($shelf[$m] === $label) {
            return $m;
        }
        if ($shelf[$lo] <= $shelf[$m]) {
            if ($shelf[$lo] <= $label && $label < $shelf[$m]) {
                $hi = $m - 1;
            } else {
                $lo = $m + 1;
            }
        } elseif ($shelf[$m] < $label && $label <= $shelf[$hi]) {
            $lo = $m + 1;
        } else {
            $hi = $m - 1;
        }
    }
    return -1;
}
