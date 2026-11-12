<?php
function daysUntilWarmer(array $temps): array {
    $n = count($temps);
    $wait = array_fill(0, $n, 0);
    $stack = [];
    for ($i = 0; $i < $n; $i++) {
        while ($stack && $temps[end($stack)] < $temps[$i]) {
            $j = array_pop($stack);
            $wait[$j] = $i - $j;
        }
        $stack[] = $i;
    }
    return $wait;
}
