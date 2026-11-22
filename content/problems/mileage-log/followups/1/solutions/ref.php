<?php
function longDays(array $miles, array $trips): array {
    $n = count($miles);
    $tree = array_fill(0, $n + 1, 0);
    $days = range(0, $n - 1);
    usort($days, fn($x, $y) => $miles[$y] <=> $miles[$x]);
    $order = array_keys($trips);
    usort($order, fn($x, $y) => $trips[$y][2] <=> $trips[$x][2]);
    $out = array_fill(0, count($trips), 0);
    $p = 0;
    foreach ($order as $j) {
        [$a, $b, $x] = $trips[$j];
        while ($p < $n && $miles[$days[$p]] >= $x) {
            for ($i = $days[$p] + 1; $i <= $n; $i += $i & -$i) {
                $tree[$i]++;
            }
            $p++;
        }
        $s = 0;
        for ($i = $b + 1; $i > 0; $i -= $i & -$i) {
            $s += $tree[$i];
        }
        for ($i = $a; $i > 0; $i -= $i & -$i) {
            $s -= $tree[$i];
        }
        $out[$j] = $s;
    }
    return $out;
}
