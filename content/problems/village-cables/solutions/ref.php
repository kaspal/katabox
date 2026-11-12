<?php
function cheapestNetwork(int $n, array $cables): int {
    $parent = range(0, $n - 1);
    $find = function (int $x) use (&$parent): int {
        while ($parent[$x] !== $x) {
            $parent[$x] = $parent[$parent[$x]];
            $x = $parent[$x];
        }
        return $x;
    };
    usort($cables, fn($p, $q) => $p[2] <=> $q[2]);
    $total = 0;
    $laid = 0;
    foreach ($cables as [$a, $b, $cost]) {
        if ($laid === $n - 1) {
            break;
        }
        $ra = $find($a);
        $rb = $find($b);
        if ($ra !== $rb) {
            $parent[$ra] = $rb;
            $total += $cost;
            $laid++;
        }
    }
    return $laid === $n - 1 ? $total : -1;
}
