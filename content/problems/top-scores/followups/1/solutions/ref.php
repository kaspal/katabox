<?php
function kthAfterEach(array $scores, int $k): array {
    $top = new SplMinHeap();
    $out = [];
    foreach ($scores as $s) {
        if ($top->count() < $k) {
            $top->insert($s);
        } elseif ($s > $top->top()) {
            $top->extract();
            $top->insert($s);
        }
        $out[] = $top->count() === $k ? $top->top() : -1;
    }
    return $out;
}
