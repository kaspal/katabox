<?php
function clubCount(int $n, array $friendships): array {
    $parent = range(0, $n - 1);
    $find = function (int $x) use (&$parent): int {
        while ($parent[$x] !== $x) {
            $parent[$x] = $parent[$parent[$x]];
            $x = $parent[$x];
        }
        return $x;
    };
    $clubs = $n;
    $out = [];
    foreach ($friendships as [$a, $b]) {
        $ra = $find($a);
        $rb = $find($b);
        if ($ra !== $rb) {
            $parent[$ra] = $rb;
            $clubs--;
        }
        $out[] = $clubs;
    }
    return $out;
}
