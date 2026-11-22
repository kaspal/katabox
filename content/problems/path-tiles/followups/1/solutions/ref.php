<?php
function tilingsMul(array $a, array $b): array {
    $c = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    for ($i = 0; $i < 3; $i++) {
        for ($j = 0; $j < 3; $j++) {
            $c[$i][$j] = ($a[$i][0] * $b[0][$j] + $a[$i][1] * $b[1][$j] + $a[$i][2] * $b[2][$j]) % 1000003;
        }
    }
    return $c;
}

function tilingsLong(int $n): int {
    $step = [[1, 1, 1], [1, 0, 0], [0, 1, 0]];
    $power = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    $e = $n;
    while ($e > 0) {
        if ($e & 1) {
            $power = tilingsMul($power, $step);
        }
        $step = tilingsMul($step, $step);
        $e >>= 1;
    }
    return $power[0][0];
}
