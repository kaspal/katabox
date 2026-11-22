<?php
function fewestInsertions(string $text): int {
    $open = 0;
    $added = 0;
    $n = strlen($text);
    for ($i = 0; $i < $n; $i++) {
        if ($text[$i] === '(') {
            $open++;
        } elseif ($text[$i] === ')') {
            if ($open > 0) {
                $open--;
            } else {
                $added++;
            }
        }
    }
    return $added + $open;
}
