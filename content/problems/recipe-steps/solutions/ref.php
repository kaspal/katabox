<?php
function stepOrder(int $n, array $before): array {
    $waiting = array_fill(0, $n, 0);
    $out = array_fill(0, $n, []);
    foreach ($before as [$a, $b]) {
        $out[$a][] = $b;
        $waiting[$b]++;
    }
    $heap = new SplMinHeap();
    for ($s = 0; $s < $n; $s++) {
        if ($waiting[$s] === 0) {
            $heap->insert($s);
        }
    }
    $order = [];
    while (!$heap->isEmpty()) {
        $s = $heap->extract();
        $order[] = $s;
        foreach ($out[$s] as $t) {
            if (--$waiting[$t] === 0) {
                $heap->insert($t);
            }
        }
    }
    return count($order) === $n ? $order : [];
}
