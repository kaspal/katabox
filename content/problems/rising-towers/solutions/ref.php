<?php
function longestRise(array $heights): int {
    $tails = [];
    foreach ($heights as $h) {
        $lo = 0;
        $hi = count($tails);
        while ($lo < $hi) {
            $mid = ($lo + $hi) >> 1;
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
