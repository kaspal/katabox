<?php
function risingRun(array $heights): int {
    $tails = [];
    foreach ($heights as $h) {
        $lo = 0;
        $hi = count($tails);
        while ($lo < $hi) {
            $mid = intdiv($lo + $hi, 2);
            if ($tails[$mid] < $h) {
                $lo = $mid + 1;
            } else {
                $hi = $mid;
            }
        }
        $tails[$lo] = $h;
    }
    return count($tails);
}
