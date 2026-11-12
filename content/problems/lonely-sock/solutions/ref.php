<?php
function loneSock(array $ids): int {
    $acc = 0;
    foreach ($ids as $x) {
        $acc ^= $x;
    }
    return $acc;
}
