<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$seen = [];
$count = 0;
$distinct = [0];
for ($p = 1; $p <= $n; $p++) {
    $id = (int) $tokens[$p];
    if (!isset($seen[$id])) {
        $seen[$id] = true;
        $count++;
    }
    $distinct[$p] = $count;
}
$q = (int) $tokens[1 + $n];
$out = [];
for ($j = 0; $j < $q; $j++) {
    $out[] = $distinct[(int) $tokens[2 + $n + $j]];
}
echo implode("\n", $out), "\n";
