<?php
function criticalBridges(int $n, array $bridges): array {
    $adj = array_fill(0, $n, []);
    foreach ($bridges as $idx => [$a, $b]) {
        $adj[$a][] = [$b, $idx];
        $adj[$b][] = [$a, $idx];
    }
    $tin = array_fill(0, $n, -1);
    $low = array_fill(0, $n, 0);
    $timer = 0;
    $out = [];
    for ($s = 0; $s < $n; $s++) {
        if ($tin[$s] !== -1) {
            continue;
        }
        $tin[$s] = $low[$s] = $timer++;
        $nodes = [$s];
        $next = [0];
        $via = [-1];
        while ($nodes !== []) {
            $top = count($nodes) - 1;
            $v = $nodes[$top];
            if ($next[$top] < count($adj[$v])) {
                [$u, $idx] = $adj[$v][$next[$top]];
                $next[$top]++;
                if ($idx === $via[$top]) {
                    continue;
                }
                if ($tin[$u] === -1) {
                    $tin[$u] = $low[$u] = $timer++;
                    $nodes[] = $u;
                    $next[] = 0;
                    $via[] = $idx;
                } else {
                    $low[$v] = min($low[$v], $tin[$u]);
                }
            } else {
                array_pop($nodes);
                array_pop($next);
                array_pop($via);
                if ($nodes !== []) {
                    $p = $nodes[count($nodes) - 1];
                    $low[$p] = min($low[$p], $low[$v]);
                    if ($low[$v] > $tin[$p]) {
                        $out[] = [min($p, $v), max($p, $v)];
                    }
                }
            }
        }
    }
    usort($out, fn ($x, $y) => $x[0] <=> $y[0] ?: $x[1] <=> $y[1]);
    return $out;
}
