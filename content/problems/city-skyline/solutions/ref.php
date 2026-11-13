<?php
function skyline(array $buildings): array {
    usort($buildings, fn ($a, $b) => $a[0] <=> $b[0]);
    $xs = [];
    foreach ($buildings as [$l, $r]) {
        $xs[$l] = true;
        $xs[$r] = true;
    }
    $xs = array_keys($xs);
    sort($xs);
    $heap = new SplMaxHeap();
    $out = [];
    $i = 0;
    $n = count($buildings);
    $last = 0;
    foreach ($xs as $x) {
        while ($i < $n && $buildings[$i][0] <= $x) {
            $heap->insert([$buildings[$i][2], $buildings[$i][1]]);
            $i++;
        }
        while (!$heap->isEmpty() && $heap->top()[1] <= $x) {
            $heap->extract();
        }
        $h = $heap->isEmpty() ? 0 : $heap->top()[0];
        if ($h !== $last) {
            $out[] = [$x, $h];
            $last = $h;
        }
    }
    return $out;
}
