<?php
function settle(array $paid): array {
    $share = array_sum($paid) / count($paid);
    return array_map(fn($p) => $p - $share, $paid);
}
