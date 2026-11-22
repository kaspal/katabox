<?php
function longestBalancedPart(string $text): int {
    $pairs = [')' => '(', ']' => '[', '}' => '{'];
    $stack = [-1];
    $best = 0;
    $n = strlen($text);
    for ($i = 0; $i < $n; $i++) {
        $c = $text[$i];
        if ($c === '(' || $c === '[' || $c === '{') {
            $stack[] = $i;
        } elseif (count($stack) > 1 && $text[$stack[count($stack) - 1]] === $pairs[$c]) {
            array_pop($stack);
            $best = max($best, $i - $stack[count($stack) - 1]);
        } else {
            $stack = [$i];
        }
    }
    return $best;
}
