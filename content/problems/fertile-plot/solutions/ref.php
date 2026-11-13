<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$rows = (int) $tokens[0];
$cols = (int) $tokens[1];
$grid = [];
$pos = 2;
for ($i = 0; $i < $rows; $i++) {
    $row = [];
    for ($j = 0; $j < $cols; $j++) {
        $row[] = (int) $tokens[$pos++];
    }
    $grid[] = $row;
}
if ($rows > $cols) {
    // Put the shorter side in the outer loops.
    $flipped = [];
    for ($j = 0; $j < $cols; $j++) {
        $line = [];
        for ($i = 0; $i < $rows; $i++) {
            $line[] = $grid[$i][$j];
        }
        $flipped[] = $line;
    }
    $grid = $flipped;
    [$rows, $cols] = [$cols, $rows];
}
$best = PHP_INT_MIN;
for ($top = 0; $top < $rows; $top++) {
    $col = array_fill(0, $cols, 0);
    for ($bottom = $top; $bottom < $rows; $bottom++) {
        $row = $grid[$bottom];
        $cur = 0; // Kadane: the best run of columns ending at j
        for ($j = 0; $j < $cols; $j++) {
            $x = ($col[$j] += $row[$j]);
            $cur = $cur > 0 ? $cur + $x : $x;
            if ($cur > $best) {
                $best = $cur;
            }
        }
    }
}
echo $best, "\n";
