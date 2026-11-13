<?php
function peakIndex(array $heights): int {
    $lo = 0;
    $hi = count($heights) - 1;
    while ($lo < $hi) {
        $m = intdiv($lo + $hi, 2);
        if ($heights[$m] < $heights[$m + 1]) {
            $lo = $m + 1;
        } else {
            $hi = $m;
        }
    }
    return $lo;
}
