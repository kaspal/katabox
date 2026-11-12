<?php
function shortNames(array $names): array {
    // Edges in one array keyed by node * 26 + letter; the root is node 0.
    $next = [];
    $passes = [0];
    foreach ($names as $s) {
        $node = 0;
        $len = strlen($s);
        for ($i = 0; $i < $len; $i++) {
            $key = $node * 26 + ord($s[$i]) - 97;
            if (!isset($next[$key])) {
                $next[$key] = count($passes);
                $passes[] = 0;
            }
            $node = $next[$key];
            $passes[$node]++;
        }
    }
    $out = [];
    foreach ($names as $s) {
        $node = 0;
        $len = strlen($s);
        $label = $s;
        for ($i = 0; $i < $len; $i++) {
            $node = $next[$node * 26 + ord($s[$i]) - 97];
            if ($passes[$node] === 1) {
                $label = substr($s, 0, $i + 1);
                break;
            }
        }
        $out[] = $label;
    }
    return $out;
}
