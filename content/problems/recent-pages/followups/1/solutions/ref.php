<?php
function favouritePages(int $capacity, array $ops, array $args): array {
    $value = [];
    $freq = [];
    $buckets = []; // $buckets[f][page] = true, in order of use
    $least = 0;
    $out = [];
    $touch = function (int $page) use (&$freq, &$buckets, &$least): void {
        $f = $freq[$page];
        unset($buckets[$f][$page]);
        if (count($buckets[$f]) === 0) {
            unset($buckets[$f]);
            if ($least === $f) {
                $least = $f + 1;
            }
        }
        $freq[$page] = $f + 1;
        $buckets[$f + 1][$page] = true;
    };
    foreach ($ops as $i => $op) {
        $a = $args[$i];
        if ($op === 'get') {
            if (array_key_exists($a[0], $value)) {
                $touch($a[0]);
                $out[] = $value[$a[0]];
            } else {
                $out[] = -1;
            }
            continue;
        }
        [$page, $v] = $a;
        if (array_key_exists($page, $value)) {
            $value[$page] = $v;
            $touch($page);
            continue;
        }
        if (count($value) === $capacity) {
            $old = array_key_first($buckets[$least]);
            unset($buckets[$least][$old]);
            if (count($buckets[$least]) === 0) {
                unset($buckets[$least]);
            }
            unset($value[$old], $freq[$old]);
        }
        $value[$page] = $v;
        $freq[$page] = 1;
        $buckets[1][$page] = true;
        $least = 1;
    }
    return $out;
}
