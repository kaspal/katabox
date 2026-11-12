<?php
function groups(int $n, array $links): int {
    $parent = range(0, $n - 1);
    $size = array_fill(0, $n, 1);
    $find = function (int $x) use (&$parent): int {
        while ($parent[$x] !== $x) {
            $parent[$x] = $parent[$parent[$x]];
            $x = $parent[$x];
        }
        return $x;
    };
    $merges = 0;
    foreach ($links as [$a, $b]) {
        $ra = $find($a);
        $rb = $find($b);
        if ($ra === $rb) {
            continue;
        }
        if ($size[$ra] < $size[$rb]) {
            [$ra, $rb] = [$rb, $ra];
        }
        $parent[$rb] = $ra;
        $size[$ra] += $size[$rb];
        $merges++;
    }
    return $n - $merges;
}
