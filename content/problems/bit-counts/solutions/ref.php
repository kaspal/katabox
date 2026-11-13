<?php
function bitCounts(int $n): array {
    $bits = [0];
    for ($i = 1; $i <= $n; $i++) {
        $bits[$i] = $bits[$i >> 1] + ($i & 1);
    }
    return $bits;
}
