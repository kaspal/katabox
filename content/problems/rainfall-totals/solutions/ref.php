<?php
function rangeTotals(array $rain, array $queries): array {
    $prefix = [0];
    foreach ($rain as $d => $r) {
        $prefix[$d + 1] = $prefix[$d] + $r;
    }
    $out = [];
    foreach ($queries as [$first, $last]) {
        $out[] = $prefix[$last + 1] - $prefix[$first];
    }
    return $out;
}
