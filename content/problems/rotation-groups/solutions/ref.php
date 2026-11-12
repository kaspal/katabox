<?php
function rotationGroups(array $codes): int {
    $seen = [];
    foreach ($codes as $code) {
        $best = $code;
        $len = strlen($code);
        for ($i = 1; $i < $len; $i++) {
            $r = substr($code, $i) . substr($code, 0, $i);
            if (strcmp($r, $best) < 0) {
                $best = $r;
            }
        }
        $seen[$best] = true;
    }
    return count($seen);
}
