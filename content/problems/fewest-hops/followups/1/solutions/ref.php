<?php
function countRoutes(int $n, array $links): int {
    $mod = 1000003;
    $adj = array_fill(0, $n, []);
    foreach ($links as [$a, $b]) {
        $adj[$a][] = $b;
        $adj[$b][] = $a;
    }
    $dist = array_fill(0, $n, -1);
    $ways = array_fill(0, $n, 0);
    $dist[0] = 0;
    $ways[0] = 1;
    $queue = [0];
    for ($head = 0; $head < count($queue); $head++) {
        $u = $queue[$head];
        foreach ($adj[$u] as $v) {
            if ($dist[$v] === -1) {
                $dist[$v] = $dist[$u] + 1;
                $ways[$v] = $ways[$u];
                $queue[] = $v;
            } elseif ($dist[$v] === $dist[$u] + 1) {
                $ways[$v] = ($ways[$v] + $ways[$u]) % $mod;
            }
        }
    }
    return $ways[$n - 1] % $mod;
}
