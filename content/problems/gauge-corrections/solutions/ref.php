<?php
function gaugeReadings(array $readings, array $ops, array $args): array {
    $n = count($readings);
    $tree = array_fill(0, $n + 1, 0);
    $add = function (int $i, int $delta) use (&$tree, $n): void {
        for ($k = $i + 1; $k <= $n; $k += $k & -$k) {
            $tree[$k] += $delta;
        }
    };
    $prefix = function (int $i) use (&$tree): int {
        $total = 0;
        for ($k = $i; $k > 0; $k -= $k & -$k) {
            $total += $tree[$k];
        }
        return $total;
    };
    $out = [];
    foreach ($ops as $j => $op) {
        $a = $args[$j];
        if ($op === "add") {
            $add($a[0], $a[2]);
            if ($a[1] + 1 < $n) {
                $add($a[1] + 1, -$a[2]);
            }
        } else {
            $out[] = $readings[$a[0]] + $prefix($a[0] + 1);
        }
    }
    return $out;
}
