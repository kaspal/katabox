<?php
function teamTotals(array $boss, array $sales): array {
    $children = array_fill(0, count($boss), []);
    $owner = 0;
    foreach ($boss as $e => $b) {
        if ($b < 0) {
            $owner = $e;
        } else {
            $children[$b][] = $e;
        }
    }
    $order = [$owner];
    for ($k = 0; $k < count($order); $k++) {
        foreach ($children[$order[$k]] as $c) {
            $order[] = $c;
        }
    }
    $total = $sales;
    for ($k = count($order) - 1; $k >= 0; $k--) {
        $e = $order[$k];
        if ($boss[$e] >= 0) {
            $total[$boss[$e]] += $total[$e];
        }
    }
    return $total;
}
