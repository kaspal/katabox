<?php
function liveCounts(array $ops, array $words): array {
    $children = [[]];
    $count = [0];
    $out = [];
    foreach ($ops as $i => $op) {
        $w = $words[$i];
        $len = strlen($w);
        if ($op === 'count') {
            $node = 0;
            for ($j = 0; $j < $len; $j++) {
                if (!isset($children[$node][$w[$j]])) {
                    $node = -1;
                    break;
                }
                $node = $children[$node][$w[$j]];
            }
            $out[] = $node === -1 ? 0 : $count[$node];
            continue;
        }
        $step = $op === 'add' ? 1 : -1;
        $node = 0;
        for ($j = 0; $j < $len; $j++) {
            $c = $w[$j];
            if (!isset($children[$node][$c])) {
                $children[] = [];
                $count[] = 0;
                $children[$node][$c] = count($count) - 1;
            }
            $node = $children[$node][$c];
            $count[$node] += $step;
        }
    }
    return $out;
}
