<?php
function clubsLeft(int $n, array $merges): array {
    $parent = range(0, $n - 1);
    $size = array_fill(0, $n, 1);
    $find = function (int $x) use (&$parent): int {
        while ($parent[$x] !== $x) {
            $parent[$x] = $parent[$parent[$x]];
            $x = $parent[$x];
        }
        return $x;
    };
    $clubs = $n;
    $out = [];
    foreach ($merges as [$a, $b]) {
        $ra = $find($a);
        $rb = $find($b);
        if ($ra !== $rb) {
            if ($size[$ra] < $size[$rb]) {
                [$ra, $rb] = [$rb, $ra];
            }
            $parent[$rb] = $ra;
            $size[$ra] += $size[$rb];
            $clubs--;
        }
        $out[] = $clubs;
    }
    return $out;
}
