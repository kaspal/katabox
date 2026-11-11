<?php
function isBalanced(string $text): bool {
    $match = [')' => '(', ']' => '[', '}' => '{'];
    $stack = [];
    $n = strlen($text);
    for ($i = 0; $i < $n; $i++) {
        $ch = $text[$i];
        if ($ch === '(' || $ch === '[' || $ch === '{') {
            $stack[] = $ch;
        } elseif (isset($match[$ch])) {
            if (count($stack) === 0 || array_pop($stack) !== $match[$ch]) {
                return false;
            }
        }
    }
    return count($stack) === 0;
}
