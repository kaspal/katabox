<?php
function mutualReach(int $n, array $links): array {
    $adj = array_fill(0, $n, []);
    $radj = array_fill(0, $n, []);
    foreach ($links as [$a, $b]) {
        $adj[$a][] = $b;
        $radj[$b][] = $a;
    }
    $seen = array_fill(0, $n, false);
    $order = [];
    for ($s = 0; $s < $n; $s++) {
        if ($seen[$s]) {
            continue;
        }
        $seen[$s] = true;
        $nodes = [$s];
        $idx = [0];
        while ($nodes !== []) {
            $top = count($nodes) - 1;
            $x = $nodes[$top];
            if ($idx[$top] < count($adj[$x])) {
                $y = $adj[$x][$idx[$top]];
                $idx[$top]++;
                if (!$seen[$y]) {
                    $seen[$y] = true;
                    $nodes[] = $y;
                    $idx[] = 0;
                }
            } else {
                array_pop($nodes);
                array_pop($idx);
                $order[] = $x;
            }
        }
    }
    $group = array_fill(0, $n, -1);
    $sizes = [];
    for ($k = count($order) - 1; $k >= 0; $k--) {
        $s = $order[$k];
        if ($group[$s] !== -1) {
            continue;
        }
        $id = count($sizes);
        $group[$s] = $id;
        $count = 0;
        $stack = [$s];
        while ($stack !== []) {
            $x = array_pop($stack);
            $count++;
            foreach ($radj[$x] as $y) {
                if ($group[$y] === -1) {
                    $group[$y] = $id;
                    $stack[] = $y;
                }
            }
        }
        $sizes[] = $count;
    }
    sort($sizes);
    return $sizes;
}
