<?php
function coldSnaps(array $temps, array $ops, array $args): array {
    $n = count($temps);
    $size = 1;
    while ($size < $n) {
        $size *= 2;
    }
    $inf = PHP_INT_MAX;
    $mn = array_fill(0, 2 * $size, $inf);
    $lazy = array_fill(0, 2 * $size, 0);
    for ($i = 0; $i < $n; $i++) {
        $mn[$size + $i] = $temps[$i];
    }
    for ($v = $size - 1; $v > 0; $v--) {
        $mn[$v] = min($mn[2 * $v], $mn[2 * $v + 1]);
    }
    $shift = function (int $v, int $lo, int $hi, int $a, int $b, int $d) use (&$shift, &$mn, &$lazy, $inf): void {
        if ($b < $lo || $hi < $a) {
            return;
        }
        if ($a <= $lo && $hi <= $b) {
            $mn[$v] += $d;
            $lazy[$v] += $d;
            return;
        }
        if ($lazy[$v] !== 0) {
            foreach ([2 * $v, 2 * $v + 1] as $c) {
                if ($mn[$c] !== $inf) {
                    $mn[$c] += $lazy[$v];
                }
                $lazy[$c] += $lazy[$v];
            }
            $lazy[$v] = 0;
        }
        $mid = intdiv($lo + $hi, 2);
        $shift(2 * $v, $lo, $mid, $a, $b, $d);
        $shift(2 * $v + 1, $mid + 1, $hi, $a, $b, $d);
        $mn[$v] = min($mn[2 * $v], $mn[2 * $v + 1]);
    };
    $coldest = function (int $v, int $lo, int $hi, int $a, int $b) use (&$coldest, &$mn, &$lazy, $inf): int {
        if ($b < $lo || $hi < $a) {
            return $inf;
        }
        if ($a <= $lo && $hi <= $b) {
            return $mn[$v];
        }
        if ($lazy[$v] !== 0) {
            foreach ([2 * $v, 2 * $v + 1] as $c) {
                if ($mn[$c] !== $inf) {
                    $mn[$c] += $lazy[$v];
                }
                $lazy[$c] += $lazy[$v];
            }
            $lazy[$v] = 0;
        }
        $mid = intdiv($lo + $hi, 2);
        return min($coldest(2 * $v, $lo, $mid, $a, $b), $coldest(2 * $v + 1, $mid + 1, $hi, $a, $b));
    };
    $out = [];
    foreach ($ops as $j => $op) {
        $a = $args[$j];
        if ($op === "shift") {
            $shift(1, 0, $size - 1, $a[0], $a[1], $a[2]);
        } else {
            $out[] = $coldest(1, 0, $size - 1, $a[0], $a[1]);
        }
    }
    return $out;
}
