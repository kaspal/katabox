<?php
function bestWeek(array $sales, int $k): int {
    $window = 0;
    for ($i = 0; $i < $k; $i++) {
        $window += $sales[$i];
    }
    $best = $window;
    $n = count($sales);
    for ($i = $k; $i < $n; $i++) {
        $window += $sales[$i] - $sales[$i - $k];
        if ($window > $best) {
            $best = $window;
        }
    }
    return $best;
}
