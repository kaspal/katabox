<?php
function tollRefunds(int $n, array $roads, int $start): array {
    $inf = PHP_INT_MAX;
    $cost = array_fill(0, $n, $inf);
    $cost[$start] = 0;
    for ($round = 0; $round < $n - 1; $round++) {
        $changed = false;
        foreach ($roads as [$a, $b, $c]) {
            if ($cost[$a] !== $inf && $cost[$a] + $c < $cost[$b]) {
                $cost[$b] = $cost[$a] + $c;
                $changed = true;
            }
        }
        if (!$changed) {
            break;
        }
    }
    foreach ($roads as [$a, $b, $c]) {
        if ($cost[$a] !== $inf && $cost[$a] + $c < $cost[$b]) {
            return [];
        }
    }
    return array_map(fn ($x) => $x === $inf ? 1000000000 : $x, $cost);
}
