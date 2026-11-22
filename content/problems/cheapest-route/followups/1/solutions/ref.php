<?php
function cheapestWithin(int $n, array $roads, int $start, int $goal, int $k): int {
    $inf = PHP_INT_MAX;
    $best = array_fill(0, $n, $inf);
    $best[$start] = 0;
    for ($r = 0; $r < $k; $r++) {
        $nxt = $best;
        $changed = false;
        foreach ($roads as [$a, $b, $t]) {
            if ($best[$a] !== $inf && $best[$a] + $t < $nxt[$b]) {
                $nxt[$b] = $best[$a] + $t;
                $changed = true;
            }
            if ($best[$b] !== $inf && $best[$b] + $t < $nxt[$a]) {
                $nxt[$a] = $best[$b] + $t;
                $changed = true;
            }
        }
        $best = $nxt;
        if (!$changed) {
            break;
        }
    }
    return $best[$goal] === $inf ? -1 : $best[$goal];
}
