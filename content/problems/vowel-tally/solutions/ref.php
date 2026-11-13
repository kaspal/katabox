<?php
function vowelTally(string $text): int {
    $count = 0;
    $n = strlen($text);
    for ($i = 0; $i < $n; $i++) {
        if (strpos("aeiou", strtolower($text[$i])) !== false) {
            $count++;
        }
    }
    return $count;
}
