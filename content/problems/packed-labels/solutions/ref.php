<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$labelCount = (int) $tokens[0];
$ends = [];
$letters = [];
for ($l = 1; $l <= $labelCount; $l++) {
    preg_match_all('/(\d+)([a-z])/', $tokens[$l], $m);
    $labelEnds = [];
    $total = 0;
    foreach ($m[1] as $count) {
        $total += (int) $count;
        $labelEnds[] = $total;
    }
    $ends[] = $labelEnds;
    $letters[] = $m[2];
}
$q = (int) $tokens[1 + $labelCount];
$pos = 2 + $labelCount;
$out = [];
for ($j = 0; $j < $q; $j++) {
    $i = (int) $tokens[$pos] - 1;
    $k = (int) $tokens[$pos + 1];
    $pos += 2;
    $e = $ends[$i];
    // First run whose end is at least k.
    $lo = 0;
    $hi = count($e) - 1;
    while ($lo < $hi) {
        $mid = ($lo + $hi) >> 1;
        if ($e[$mid] >= $k) {
            $hi = $mid;
        } else {
            $lo = $mid + 1;
        }
    }
    $out[] = $letters[$i][$lo];
}
echo implode("\n", $out), "\n";
