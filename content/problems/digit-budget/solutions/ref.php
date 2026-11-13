<?php
function digitBudget(int $limit, int $total): int {
    $digits = array_map('intval', str_split((string) $limit));
    $n = count($digits);
    $memo = [];
    $free = function (int $pos, int $s) use (&$free, &$memo, $n, $total): int {
        if ($s > $total) {
            return 0;
        }
        if ($pos === $n) {
            return $s === $total ? 1 : 0;
        }
        $key = $pos * 1000 + $s;
        if (!isset($memo[$key])) {
            $count = 0;
            for ($x = 0; $x <= 9; $x++) {
                $count += $free($pos + 1, $s + $x);
            }
            $memo[$key] = $count;
        }
        return $memo[$key];
    };
    $count = 0;
    $s = 0;
    foreach ($digits as $pos => $top) {
        for ($x = 0; $x < $top; $x++) {
            $count += $free($pos + 1, $s + $x);
        }
        $s += $top;
    }
    return $count + ($s === $total ? 1 : 0);
}
