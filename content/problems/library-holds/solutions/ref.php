<?php
function holds(array $ops, array $args): array {
    $line = [];
    $head = [];
    $waiting = [];
    $tickets = 0;
    $reports = [];
    foreach ($ops as $i => $op) {
        $book = $args[$i][0];
        if ($op === 'hold') {
            $member = $args[$i][1];
            $tickets++;
            $waiting["$book:$member"] = $tickets;
            $line[$book][] = [$member, $tickets];
            $head[$book] ??= 0;
        } elseif ($op === 'cancel') {
            unset($waiting["$book:{$args[$i][1]}"]);
        } else {
            $got = -1;
            $len = isset($line[$book]) ? count($line[$book]) : 0;
            while ($len > 0 && $head[$book] < $len) {
                [$m, $t] = $line[$book][$head[$book]];
                $head[$book]++;
                if (($waiting["$book:$m"] ?? 0) === $t) {
                    unset($waiting["$book:$m"]);
                    $got = $m;
                    break;
                }
            }
            $reports[] = $got;
        }
    }
    return $reports;
}
