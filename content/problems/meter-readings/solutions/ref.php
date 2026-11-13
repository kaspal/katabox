<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$prev = (int) $tokens[1];
$total = $prev;
$rises = 0;
$run = 1;
$longest = 1;
for ($i = 2; $i <= $n; $i++) {
    $x = (int) $tokens[$i];
    $total += $x;
    if ($x === $prev) {
        $run++;
        if ($run > $longest) {
            $longest = $run;
        }
    } else {
        if ($x > $prev) {
            $rises++;
        }
        $run = 1;
    }
    $prev = $x;
}
echo "$total $rises $longest\n";
