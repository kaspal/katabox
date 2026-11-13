<?php
function tripTotals(array $miles, array $trips): array {
    $prefix = [0];
    foreach ($miles as $i => $m) {
        $prefix[] = $prefix[$i] + $m;
    }
    $out = [];
    foreach ($trips as [$a, $b]) {
        $out[] = $prefix[$b + 1] - $prefix[$a];
    }
    return $out;
}
