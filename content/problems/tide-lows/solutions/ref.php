<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$n = (int) $tokens[$pos++];
$levels = [];
for ($i = 0; $i < $n; $i++) {
    $levels[] = (int) $tokens[$pos++];
}
// low[k][i]: the lowest of the 2^k readings starting at i.
$low = [$levels];
for ($k = 1; (1 << $k) <= $n; $k++) {
    $prev = $low[$k - 1];
    $half = 1 << ($k - 1);
    $len = $n - (1 << $k) + 1;
    $row = [];
    for ($i = 0; $i < $len; $i++) {
        $a = $prev[$i];
        $b = $prev[$i + $half];
        $row[] = $a < $b ? $a : $b;
    }
    $low[] = $row;
}
// lg[len]: the largest k with 2^k <= len.
$lg = [0, 0];
for ($i = 2; $i <= $n; $i++) {
    $lg[$i] = $lg[$i >> 1] + 1;
}
$q = (int) $tokens[$pos++];
$out = [];
for ($i = 0; $i < $q; $i++) {
    $l = (int) $tokens[$pos++] - 1;
    $r = (int) $tokens[$pos++] - 1;
    $k = $lg[$r - $l + 1];
    $a = $low[$k][$l];
    $b = $low[$k][$r - (1 << $k) + 1];
    $out[] = $a < $b ? $a : $b;
}
echo implode("\n", $out), "\n";
