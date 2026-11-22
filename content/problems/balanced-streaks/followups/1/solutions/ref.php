<?php
function longestBalanced(string $results): int {
    $first = [0 => -1];
    $score = 0;
    $best = 0;
    $n = strlen($results);
    for ($i = 0; $i < $n; $i++) {
        $c = $results[$i];
        if ($c === 'W') {
            $score++;
        } elseif ($c === 'L') {
            $score--;
        }
        if (array_key_exists($score, $first)) {
            $best = max($best, $i - $first[$score]);
        } else {
            $first[$score] = $i;
        }
    }
    return $best;
}
