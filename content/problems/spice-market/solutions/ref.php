<?php
function bestHaul(array $weights, array $prices, int $capacity): float {
    $order = array_keys($weights);
    usort($order, fn ($i, $j) => $prices[$j] * $weights[$i] <=> $prices[$i] * $weights[$j]);
    $whole = 0;
    $room = $capacity;
    foreach ($order as $i) {
        if ($weights[$i] <= $room) {
            $whole += $prices[$i];
            $room -= $weights[$i];
        } else {
            return $whole + $prices[$i] * $room / $weights[$i];
        }
    }
    return (float) $whole;
}
