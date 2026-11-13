<?php
function gradeBands(array $scores): array {
    $bands = [];
    foreach ($scores as $s) {
        if ($s >= 90) {
            $bands[] = "A";
        } elseif ($s >= 80) {
            $bands[] = "B";
        } elseif ($s >= 70) {
            $bands[] = "C";
        } elseif ($s >= 60) {
            $bands[] = "D";
        } else {
            $bands[] = "F";
        }
    }
    return $bands;
}
