<?php
function prefixCounts(array $titles, array $typed): array {
    // Edges in one array keyed by node * 26 + letter; the root is node 0.
    $next = [];
    $passes = [0];
    foreach ($titles as $t) {
        $node = 0;
        $len = strlen($t);
        for ($i = 0; $i < $len; $i++) {
            $key = $node * 26 + ord($t[$i]) - 97;
            if (!isset($next[$key])) {
                $next[$key] = count($passes);
                $passes[] = 0;
            }
            $node = $next[$key];
            $passes[$node]++;
        }
    }
    $out = [];
    foreach ($typed as $p) {
        $node = 0;
        $len = strlen($p);
        for ($i = 0; $i < $len && $node >= 0; $i++) {
            $node = $next[$node * 26 + ord($p[$i]) - 97] ?? -1;
        }
        $out[] = $node < 0 ? 0 : $passes[$node];
    }
    return $out;
}
