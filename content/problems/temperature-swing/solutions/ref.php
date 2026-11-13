<?php
function temperatureSwing(array $readings): int {
    $warmest = $readings[0];
    $coldest = $readings[0];
    foreach ($readings as $r) {
        if ($r > $warmest) {
            $warmest = $r;
        }
        if ($r < $coldest) {
            $coldest = $r;
        }
    }
    return $warmest - $coldest;
}
