<?php
function plantings(int $length, array $widths): int {
    $mod = 1000000007;
    $k = count($widths);
    $ways = array_fill(0, ($length + 1) * $k, 0);
    $total = array_fill(0, $length + 1, 0);
    $total[0] = 1;
    for ($x = 1; $x <= $length; $x++) {
        $sum = 0;
        foreach ($widths as $kind => $w) {
            if ($w <= $x) {
                $v = ($total[$x - $w] - $ways[($x - $w) * $k + $kind] + $mod) % $mod;
                $ways[$x * $k + $kind] = $v;
                $sum += $v;
            }
        }
        $total[$x] = $sum % $mod;
    }
    return $total[$length];
}
