<?php
function twoTeams(int $n, array $rivalries): bool {
    $adj = array_fill(0, $n, []);
    foreach ($rivalries as [$a, $b]) {
        $adj[$a][] = $b;
        $adj[$b][] = $a;
    }
    $colour = array_fill(0, $n, -1);
    for ($s = 0; $s < $n; $s++) {
        if ($colour[$s] !== -1) {
            continue;
        }
        $colour[$s] = 0;
        $queue = [$s];
        for ($head = 0; $head < count($queue); $head++) {
            $x = $queue[$head];
            foreach ($adj[$x] as $y) {
                if ($colour[$y] === -1) {
                    $colour[$y] = 1 - $colour[$x];
                    $queue[] = $y;
                } elseif ($colour[$y] === $colour[$x]) {
                    return false;
                }
            }
        }
    }
    return true;
}
