<?php
function spanSums(array $rain, array $queries): array {
    $prefix = [0];
    foreach ($rain as $x) {
        $prefix[] = end($prefix) + $x;
    }
    return array_map(fn($q) => $prefix[$q[1] + 1] - $prefix[$q[0]], $queries);
}
