<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$r = (int) $tokens[0];
$c = (int) $tokens[1];
$rowTrees = array_fill(0, $r, 0);
$colTrees = array_fill(0, $c, 0);
for ($i = 0; $i < $r; $i++) {
    $row = $tokens[2 + $i];
    $rowTrees[$i] = substr_count($row, '#');
    for ($j = 0; $j < $c; $j++) {
        if ($row[$j] === '#') {
            $colTrees[$j]++;
        }
    }
}
$q = (int) $tokens[2 + $r];
$pos = 3 + $r;
$out = [];
for ($k = 0; $k < $q; $k++) {
    $index = (int) $tokens[$pos + 1];
    $out[] = $tokens[$pos] === 'row' ? $rowTrees[$index] : $colTrees[$index];
    $pos += 2;
}
echo implode("\n", $out), "\n";
