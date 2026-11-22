<?php
function rowLedger(array $stock, array $ops, array $args): array {
    $n = count($stock);
    $b1 = array_fill(0, $n + 1, 0);
    $b2 = array_fill(0, $n + 1, 0);
    $add = function (array &$t, int $i, int $v) use ($n): void {
        for (; $i <= $n; $i += $i & -$i) {
            $t[$i] += $v;
        }
    };
    $query = function (array &$t, int $i): int {
        $s = 0;
        for (; $i > 0; $i -= $i & -$i) {
            $s += $t[$i];
        }
        return $s;
    };
    $rangeAdd = function (int $a, int $b, int $v) use (&$b1, &$b2, $add): void {
        $add($b1, $a, $v);
        $add($b1, $b + 1, -$v);
        $add($b2, $a, $v * ($a - 1));
        $add($b2, $b + 1, -$v * $b);
    };
    $prefix = function (int $i) use (&$b1, &$b2, $query): int {
        return $query($b1, $i) * $i - $query($b2, $i);
    };
    foreach ($stock as $i => $s) {
        $rangeAdd($i + 1, $i + 1, $s);
    }
    $out = [];
    foreach ($ops as $k => $op) {
        $a = $args[$k];
        if ($op === 'add') {
            $rangeAdd($a[0] + 1, $a[1] + 1, $a[2]);
        } else {
            $out[] = $prefix($a[1] + 1) - $prefix($a[0]);
        }
    }
    return $out;
}
