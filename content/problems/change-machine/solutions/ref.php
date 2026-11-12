<?php
function fewestCoins(array $coins, int $amount): int {
    $inf = $amount + 1;
    $best = array_fill(0, $amount + 1, $inf);
    $best[0] = 0;
    for ($x = 1; $x <= $amount; $x++) {
        $b = $inf;
        foreach ($coins as $c) {
            if ($c <= $x && $best[$x - $c] + 1 < $b) {
                $b = $best[$x - $c] + 1;
            }
        }
        $best[$x] = $b;
    }
    return $best[$amount] <= $amount ? $best[$amount] : -1;
}
