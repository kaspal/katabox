<?php
function shop(array $ops, array $args): array {
    $shelf = [];
    $reports = [];
    foreach ($ops as $i => $op) {
        $item = $args[$i][0];
        $have = $shelf[$item] ?? 0;
        if ($op === 'stock') {
            $shelf[$item] = $have + $args[$i][1];
        } elseif ($op === 'sell') {
            $sold = min($args[$i][1], $have);
            $shelf[$item] = $have - $sold;
            $reports[] = $sold;
        } else {
            $reports[] = $have;
        }
    }
    return $reports;
}
