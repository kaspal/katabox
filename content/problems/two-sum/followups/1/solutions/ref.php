<?php
function countPairs(array $nums, int $target): int {
    $seen = [];
    $pairs = 0;
    foreach ($nums as $n) {
        $pairs += $seen[$target - $n] ?? 0;
        $seen[$n] = ($seen[$n] ?? 0) + 1;
    }
    return $pairs;
}
