<?php
function countPattern(string $text, string $pattern): int {
    $m = strlen($pattern);
    $pi = array_fill(0, $m, 0);
    $k = 0;
    for ($i = 1; $i < $m; $i++) {
        while ($k > 0 && $pattern[$i] !== $pattern[$k]) {
            $k = $pi[$k - 1];
        }
        if ($pattern[$i] === $pattern[$k]) {
            $k++;
        }
        $pi[$i] = $k;
    }
    $count = 0;
    $k = 0;
    $n = strlen($text);
    for ($i = 0; $i < $n; $i++) {
        $c = $text[$i];
        while ($k > 0 && $c !== $pattern[$k]) {
            $k = $pi[$k - 1];
        }
        if ($c === $pattern[$k]) {
            $k++;
        }
        if ($k === $m) {
            $count++;
            $k = $pi[$k - 1];
        }
    }
    return $count;
}
