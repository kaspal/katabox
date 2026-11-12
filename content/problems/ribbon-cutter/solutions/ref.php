<?php
function longestPiece(array $ribbons, int $pieces): int {
    $count = function (int $len) use ($ribbons): int {
        $c = 0;
        foreach ($ribbons as $r) {
            $c += intdiv($r, $len);
        }
        return $c;
    };
    if ($count(1) < $pieces) {
        return 0;
    }
    $lo = 1;
    $hi = max($ribbons);
    while ($lo < $hi) {
        $mid = intdiv($lo + $hi + 1, 2);
        if ($count($mid) >= $pieces) {
            $lo = $mid;
        } else {
            $hi = $mid - 1;
        }
    }
    return $lo;
}
