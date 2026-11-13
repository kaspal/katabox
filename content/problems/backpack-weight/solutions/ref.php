<?php
function bestPack(array $weights, array $values, int $capacity): int {
    $best = array_fill(0, $capacity + 1, 0);
    foreach ($weights as $i => $w) {
        $v = $values[$i];
        for ($c = $capacity; $c >= $w; $c--) {
            if ($best[$c - $w] + $v > $best[$c]) {
                $best[$c] = $best[$c - $w] + $v;
            }
        }
    }
    return $best[$capacity];
}
