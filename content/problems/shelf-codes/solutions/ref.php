<?php
function countBelow(array $codes, array $queries): array {
    $out = [];
    foreach ($queries as $x) {
        $lo = 0;
        $hi = count($codes);
        while ($lo < $hi) {
            $mid = ($lo + $hi) >> 1;
            if ($codes[$mid] < $x) {
                $lo = $mid + 1;
            } else {
                $hi = $mid;
            }
        }
        $out[] = $lo;
    }
    return $out;
}
