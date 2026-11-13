<?php
function canCross(array $jumps): bool {
    $reach = 0;
    foreach ($jumps as $i => $j) {
        if ($i > $reach) {
            return false;
        }
        $reach = max($reach, $i + $j);
    }
    return true;
}
