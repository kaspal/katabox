<?php
function bestXorPair(array $codes): int {
    $children = [[0, 0]];
    $best = 0;
    foreach ($codes as $i => $x) {
        if ($i > 0) {
            $node = 0;
            $value = 0;
            for ($bit = 19; $bit >= 0; $bit--) {
                $b = ($x >> $bit) & 1;
                if ($children[$node][$b ^ 1] !== 0) {
                    $value |= 1 << $bit;
                    $node = $children[$node][$b ^ 1];
                } else {
                    $node = $children[$node][$b];
                }
            }
            $best = max($best, $value);
        }
        $node = 0;
        for ($bit = 19; $bit >= 0; $bit--) {
            $b = ($x >> $bit) & 1;
            if ($children[$node][$b] === 0) {
                $children[$node][$b] = count($children);
                $children[] = [0, 0];
            }
            $node = $children[$node][$b];
        }
    }
    return $best;
}
