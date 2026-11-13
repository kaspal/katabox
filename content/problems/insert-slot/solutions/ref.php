<?php
function insertSlot(array $slots, array $fresh): array {
    [$s, $e] = $fresh;
    $out = [];
    $i = 0;
    $n = count($slots);
    while ($i < $n && $slots[$i][1] < $s) {
        $out[] = $slots[$i++];
    }
    while ($i < $n && $slots[$i][0] <= $e) {
        $s = min($s, $slots[$i][0]);
        $e = max($e, $slots[$i][1]);
        $i++;
    }
    $out[] = [$s, $e];
    while ($i < $n) {
        $out[] = $slots[$i++];
    }
    return $out;
}
