<?php
function daysUntilWarmerAround(array $temps): array {
    $n = count($temps);
    $wait = array_fill(0, $n, 0);
    $stack = [];
    for ($k = 0; $k < 2 * $n; $k++) {
        $i = $k % $n;
        while ($stack && $temps[end($stack)] < $temps[$i]) {
            $d = array_pop($stack);
            $w = ($i - $d + $n) % $n;
            $wait[$d] = $w === 0 ? $n : $w;
        }
        if ($k < $n) {
            $stack[] = $i;
        }
    }
    return $wait;
}
