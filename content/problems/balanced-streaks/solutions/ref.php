<?php
function balancedStretches(string $results): int {
    $seen = [0 => 1];
    $score = 0;
    $count = 0;
    $n = strlen($results);
    for ($i = 0; $i < $n; $i++) {
        $c = $results[$i];
        if ($c === 'W') {
            $score++;
        } elseif ($c === 'L') {
            $score--;
        }
        $count += $seen[$score] ?? 0;
        $seen[$score] = ($seen[$score] ?? 0) + 1;
    }
    return $count;
}
