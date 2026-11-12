<?php
function isMirror(string $text): bool {
    $i = 0;
    $j = strlen($text) - 1;
    while ($i < $j) {
        if (!ctype_alnum($text[$i])) {
            $i++;
        } elseif (!ctype_alnum($text[$j])) {
            $j--;
        } elseif (strtolower($text[$i]) !== strtolower($text[$j])) {
            return false;
        } else {
            $i++;
            $j--;
        }
    }
    return true;
}
