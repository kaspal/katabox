<?php
function usersWithAll(array $users, array $queries): array {
    $top = max(max($users), max($queries), 1);
    $bits = 0;
    while ((1 << $bits) <= $top) {
        $bits++;
    }
    $size = 1 << $bits;
    $count = array_fill(0, $size, 0);
    foreach ($users as $u) {
        $count[$u]++;
    }
    for ($b = 0; $b < $bits; $b++) {
        $bit = 1 << $b;
        for ($m = 0; $m < $size; $m++) {
            if (!($m & $bit)) {
                $count[$m] += $count[$m | $bit];
            }
        }
    }
    return array_map(fn ($q) => $count[$q], $queries);
}
