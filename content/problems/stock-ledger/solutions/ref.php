<?php
function stockLedger(array $stock, array $ops, array $args): array {
    $n = count($stock);
    $tree = array_fill(0, $n + 1, 0);
    $current = $stock;
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
    foreach ($stock as $i => $v) {
        $add($i, $v);
    }
    $out = [];
    foreach ($ops as $j => $op) {
        [$x, $y] = $args[$j];
        if ($op === "set") {
            $add($x, $y - $current[$x]);
            $current[$x] = $y;
        } else {
            $out[] = $prefix($y + 1) - $prefix($x);
        }
    }
    return $out;
}
