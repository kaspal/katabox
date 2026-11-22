<?php
function mostVowels(array $words): int {
    $best = 0;
    foreach ($words as $w) {
        $count = preg_match_all('/[aeiou]/', $w);
        $best = max($best, $count);
    }
    return $best;
}
