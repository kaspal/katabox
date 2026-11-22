<?php
function tilingsApart(int $n): int {
    $mod = 1000000007;
    $a = array_fill(0, $n + 1, 0);
    $b = array_fill(0, $n + 1, 0);
    $a[0] = 1;
    for ($i = 1; $i <= $n; $i++) {
        $v = $a[$i - 1] + $b[$i - 1];
        if ($i >= 2) {
            $v += $a[$i - 2] + $b[$i - 2];
        }
        $a[$i] = $v % $mod;
        if ($i >= 3) {
            $b[$i] = $a[$i - 3];
        }
    }
    return ($a[$n] + $b[$n]) % $mod;
}
