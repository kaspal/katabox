<?php
function largestPoster(array $panels): int {
    $stack = [];
    $best = 0;
    $n = count($panels);
    for ($i = 0; $i <= $n; $i++) {
        $h = $i < $n ? $panels[$i] : 0;
        while ($stack && $panels[end($stack)] > $h) {
            $k = array_pop($stack);
            $width = $stack ? $i - end($stack) - 1 : $i;
            $best = max($best, $panels[$k] * $width);
        }
        $stack[] = $i;
    }
    return $best;
}
