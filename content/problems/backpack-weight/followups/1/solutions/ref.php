<?php
function bestPackCopies(array $weights, array $values, array $counts, int $capacity): int {
    $best = array_fill(0, $capacity + 1, 0);
    foreach ($weights as $i => $w) {
        $c = $counts[$i];
        $k = 1;
        while ($c > 0) {
            $take = min($k, $c);
            $bw = $w * $take;
            $bv = $values[$i] * $take;
            for ($cap = $capacity; $cap >= $bw; $cap--) {
                if ($best[$cap - $bw] + $bv > $best[$cap]) {
                    $best[$cap] = $best[$cap - $bw] + $bv;
                }
            }
            $c -= $take;
            $k *= 2;
        }
    }
    return $best[$capacity];
}
