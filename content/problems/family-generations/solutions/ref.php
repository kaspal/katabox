<?php
function generations(array $parent): int {
    $children = array_fill(0, count($parent), []);
    $founder = 0;
    foreach ($parent as $i => $p) {
        if ($p < 0) {
            $founder = $i;
        } else {
            $children[$p][] = $i;
        }
    }
    $level = [$founder];
    $count = 0;
    while ($level) {
        $count++;
        $next = [];
        foreach ($level as $p) {
            foreach ($children[$p] as $c) {
                $next[] = $c;
            }
        }
        $level = $next;
    }
    return $count;
}
