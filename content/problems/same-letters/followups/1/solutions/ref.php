<?php
function twinsAfterChanges(string $a, string $b, array $positions, string $letters): array {
    $diff = array_fill(0, 26, 0);
    for ($i = 0; $i < strlen($a); $i++) {
        $diff[ord($a[$i]) - 97]++;
    }
    for ($i = 0; $i < strlen($b); $i++) {
        $diff[ord($b[$i]) - 97]--;
    }
    $off = 0;
    foreach ($diff as $d) {
        if ($d !== 0) {
            $off++;
        }
    }
    $tiles = $b;
    $out = [];
    foreach ($positions as $i => $p) {
        foreach ([[$tiles[$p], 1], [$letters[$i], -1]] as [$letter, $step]) {
            $k = ord($letter) - 97;
            $before = $diff[$k] !== 0 ? 1 : 0;
            $diff[$k] += $step;
            $off += ($diff[$k] !== 0 ? 1 : 0) - $before;
        }
        $tiles[$p] = $letters[$i];
        $out[] = $off === 0;
    }
    return $out;
}
