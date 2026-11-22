<?php
function clubsWithUndo(int $n, array $ops): array {
    $parent = range(0, $n - 1);
    $size = array_fill(0, $n, 1);
    $history = [];
    $clubs = $n;
    $out = [];
    foreach ($ops as $op) {
        if ($op[0] === 0) {
            if ($history) {
                $rb = array_pop($history);
                if ($rb !== -1) {
                    $ra = $parent[$rb];
                    $size[$ra] -= $size[$rb];
                    $parent[$rb] = $rb;
                    $clubs++;
                }
            }
        } else {
            $ra = $op[1];
            while ($parent[$ra] !== $ra) {
                $ra = $parent[$ra];
            }
            $rb = $op[2];
            while ($parent[$rb] !== $rb) {
                $rb = $parent[$rb];
            }
            if ($ra === $rb) {
                $history[] = -1;
            } else {
                if ($size[$ra] < $size[$rb]) {
                    [$ra, $rb] = [$rb, $ra];
                }
                $parent[$rb] = $ra;
                $size[$ra] += $size[$rb];
                $clubs--;
                $history[] = $rb;
            }
        }
        $out[] = $clubs;
    }
    return $out;
}
