<?php
function evaluate(string $expression): int {
    $pos = 0;
    $n = strlen($expression);
    $peek = function () use (&$pos, $expression, $n): string {
        while ($pos < $n && $expression[$pos] === ' ') {
            $pos++;
        }
        return $pos < $n ? $expression[$pos] : '';
    };
    $expr = null;
    $factor = function () use (&$factor, &$expr, &$pos, $peek, $expression, $n): int {
        $c = $peek();
        if ($c === '-') {
            $pos++;
            return -$factor();
        }
        if ($c === '(') {
            $pos++;
            $v = $expr();
            $peek();
            $pos++; // ")"
            return $v;
        }
        $start = $pos;
        while ($pos < $n && ctype_digit($expression[$pos])) {
            $pos++;
        }
        return (int) substr($expression, $start, $pos - $start);
    };
    $term = function () use ($factor, $peek, &$pos): int {
        $v = $factor();
        while ($peek() === '*') {
            $pos++;
            $v *= $factor();
        }
        return $v;
    };
    $expr = function () use ($term, $peek, &$pos): int {
        $v = $term();
        for ($c = $peek(); $c === '+' || $c === '-'; $c = $peek()) {
            $pos++;
            $w = $term();
            $v = $c === '+' ? $v + $w : $v - $w;
        }
        return $v;
    };
    return $expr();
}
