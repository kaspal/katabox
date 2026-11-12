<?php
function fewestHops(int $n, array $links): int {
    $near = array_fill(0, $n, []);
    foreach ($links as [$a, $b]) {
        $near[$a][] = $b;
        $near[$b][] = $a;
    }
    $dist = array_fill(0, $n, -1);
    $dist[0] = 0;
    $queue = [0];
    for ($h = 0; $h < count($queue); $h++) {
        $v = $queue[$h];
        foreach ($near[$v] as $u) {
            if ($dist[$u] === -1) {
                $dist[$u] = $dist[$v] + 1;
                $queue[] = $u;
            }
        }
    }
    return $dist[$n - 1];
}
