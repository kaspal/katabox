<?php
function longestWithKLetters(string $text, int $k): int {
    $count = [];
    $best = 0;
    $start = 0;
    $n = strlen($text);
    for ($end = 0; $end < $n; $end++) {
        $count[$text[$end]] = ($count[$text[$end]] ?? 0) + 1;
        while (count($count) > $k) {
            $s = $text[$start++];
            if (--$count[$s] === 0) {
                unset($count[$s]);
            }
        }
        $best = max($best, $end - $start + 1);
    }
    return $best;
}
