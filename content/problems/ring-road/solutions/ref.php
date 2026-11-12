<?php
function ringLength(int $n, array $roads): int {
    $near = array_fill(0, $n, []);
    foreach ($roads as [$a, $b]) {
        $near[$a][] = $b;
        $near[$b][] = $a;
    }
    $degree = array_map('count', $near);
    $removed = array_fill(0, $n, false);
    $queue = [];
    for ($v = 0; $v < $n; $v++) {
        if ($degree[$v] === 1) {
            $queue[] = $v;
        }
    }
    $left = $n;
    for ($h = 0; $h < count($queue); $h++) {
        $v = $queue[$h];
        $removed[$v] = true;
        $left--;
        foreach ($near[$v] as $u) {
            if (!$removed[$u] && --$degree[$u] === 1) {
                $queue[] = $u;
            }
        }
    }
    return $left;
}
