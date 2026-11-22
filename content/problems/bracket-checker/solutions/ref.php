<?php
function bracketsOk(string $code): bool {
    $pairs = [')' => '(', ']' => '[', '}' => '{'];
    $stack = [];
    for ($i = 0; $i < strlen($code); $i++) {
        $c = $code[$i];
        if ($c === '(' || $c === '[' || $c === '{') {
            $stack[] = $c;
        } elseif (array_pop($stack) !== $pairs[$c]) {
            return false;
        }
    }
    return count($stack) === 0;
}
