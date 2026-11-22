<?php
function raceOrder(array $names, array $times): array {
    $order = array_keys($names);
    usort($order, fn($a, $b) => [$times[$a], $names[$a]] <=> [$times[$b], $names[$b]]);
    return array_map(fn($i) => $names[$i], $order);
}
