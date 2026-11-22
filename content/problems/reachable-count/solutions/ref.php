<?php
function reachableCount(int $n, array $doors, int $start): int {
    $adj = array_fill(0, $n, []);
    foreach ($doors as [$a, $b]) {
        $adj[$a][] = $b;
        $adj[$b][] = $a;
    }
    $seen = array_fill(0, $n, false);
    $seen[$start] = true;
    $queue = [$start];
    for ($head = 0; $head < count($queue); $head++) {
        foreach ($adj[$queue[$head]] as $u) {
            if (!$seen[$u]) {
                $seen[$u] = true;
                $queue[] = $u;
            }
        }
    }
    return count($queue);
}
