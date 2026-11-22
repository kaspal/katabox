<?php
function tillTotal(array $prices): int {
    $total = 0;
    foreach ($prices as $p) {
        $total += (int) str_replace('.', '', $p);
    }
    return $total;
}
