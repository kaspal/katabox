<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$t = (int) $tokens[$pos++];
$out = [];
for ($c = 0; $c < $t; $c++) {
    $n = (int) $tokens[$pos++];
    $k = (int) $tokens[$pos++];
    $values = array_map('intval', array_slice($tokens, $pos, $n));
    $pos += $n;
    sort($values);
    // Two pointers on the sorted values: if the smallest and the largest fit, the
    // smallest fits with everything in between too.
    $count = 0;
    $i = 0;
    $j = $n - 1;
    while ($i < $j) {
        if ($values[$i] + $values[$j] <= $k) {
            $count += $j - $i;
            $i++;
        } else {
            $j--;
        }
    }
    $out[] = $count;
}
echo implode("\n", $out), "\n";
