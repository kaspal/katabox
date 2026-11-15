<?php
$lines = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$r = (int) $lines[0];
$c = (int) $lines[1];
// height[x]: how many free squares stand in column x, ending at the current row. Each
// row then is a skyline, and the best banner whose bottom edge is on this row is the
// largest rectangle under it: a stack of columns of increasing height.
$height = array_fill(0, $c + 1, 0); // a 0 at the end empties the stack
$best = 0;
for ($y = 0; $y < $r; $y++) {
    $row = $lines[2 + $y];
    for ($x = 0; $x < $c; $x++) {
        $height[$x] = $row[$x] === '.' ? $height[$x] + 1 : 0;
    }
    $stack = [];
    $top = 0;
    for ($x = 0; $x <= $c; $x++) {
        $h = $height[$x];
        while ($top > 0 && $height[$stack[$top - 1]] >= $h) {
            $tall = $height[$stack[--$top]];
            $left = $top > 0 ? $stack[$top - 1] + 1 : 0;
            if ($tall * ($x - $left) > $best) {
                $best = $tall * ($x - $left);
            }
        }
        $stack[$top++] = $x;
    }
}
echo $best, "\n";
