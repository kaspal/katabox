<?php
function recentPages(int $capacity, array $ops, array $args): array {
    // PHP arrays keep insertion order: unset and re-insert to mark a page as used.
    $cache = [];
    $out = [];
    foreach ($ops as $i => $op) {
        $page = $args[$i][0];
        if ($op === 'get') {
            if (array_key_exists($page, $cache)) {
                $value = $cache[$page];
                unset($cache[$page]);
                $cache[$page] = $value;
                $out[] = $value;
            } else {
                $out[] = -1;
            }
        } else {
            unset($cache[$page]);
            $cache[$page] = $args[$i][1];
            if (count($cache) > $capacity) {
                unset($cache[array_key_first($cache)]);
            }
        }
    }
    return $out;
}
