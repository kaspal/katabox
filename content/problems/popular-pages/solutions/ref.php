<?php
function popularPages(int $capacity, array $ops, array $args): array {
    $value = [];
    $count = [];
    // count => [page => true], in insertion (= last use) order.
    $bucket = [];
    $minCount = 0;
    $out = [];
    $use = function (int $page) use (&$count, &$bucket, &$minCount): void {
        $c = $count[$page];
        unset($bucket[$c][$page]);
        if (!$bucket[$c]) {
            unset($bucket[$c]);
            if ($minCount === $c) {
                $minCount = $c + 1;
            }
        }
        $count[$page] = $c + 1;
        $bucket[$c + 1][$page] = true;
    };
    foreach ($ops as $i => $op) {
        $page = $args[$i][0];
        if ($op === 'get') {
            if (array_key_exists($page, $value)) {
                $use($page);
                $out[] = $value[$page];
            } else {
                $out[] = -1;
            }
        } elseif (array_key_exists($page, $value)) {
            $value[$page] = $args[$i][1];
            $use($page);
        } else {
            if (count($value) === $capacity) {
                $old = array_key_first($bucket[$minCount]);
                unset($bucket[$minCount][$old]);
                if (!$bucket[$minCount]) {
                    unset($bucket[$minCount]);
                }
                unset($value[$old], $count[$old]);
            }
            $value[$page] = $args[$i][1];
            $count[$page] = 1;
            $bucket[1][$page] = true;
            $minCount = 1;
        }
    }
    return $out;
}
