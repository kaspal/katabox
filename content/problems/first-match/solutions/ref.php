<?php
function firstMatches(array $words, array $prefixes): array {
    $sorted = array_values(array_unique($words));
    sort($sorted, SORT_STRING);
    // Edges in one array keyed by node * 26 + letter; the root is node 0.
    $next = [];
    $best = [''];
    foreach ($sorted as $w) {
        $node = 0;
        $len = strlen($w);
        for ($i = 0; $i < $len; $i++) {
            $key = $node * 26 + ord($w[$i]) - 97;
            if (!isset($next[$key])) {
                $next[$key] = count($best);
                $best[] = $w;
            }
            $node = $next[$key];
        }
    }
    $out = [];
    foreach ($prefixes as $p) {
        $node = 0;
        $len = strlen($p);
        for ($i = 0; $i < $len && $node >= 0; $i++) {
            $node = $next[$node * 26 + ord($p[$i]) - 97] ?? -1;
        }
        $out[] = $node < 0 ? '' : $best[$node];
    }
    return $out;
}
