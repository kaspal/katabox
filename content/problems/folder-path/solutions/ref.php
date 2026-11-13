<?php
function cleanPath(string $path): string {
    $stack = [];
    foreach (explode('/', $path) as $part) {
        if ($part === '..') {
            array_pop($stack);
        } elseif ($part !== '' && $part !== '.') {
            $stack[] = $part;
        }
    }
    return '/' . implode('/', $stack);
}
