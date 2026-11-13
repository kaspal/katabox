<?php
function fuelStops(int $target, int $fuel, array $stations): int {
    $heap = new SplMaxHeap();
    $reach = $fuel;
    $stops = 0;
    $stations[] = [$target, 0];
    foreach ($stations as [$position, $litres]) {
        while ($reach < $position) {
            if ($heap->isEmpty()) {
                return -1;
            }
            $reach += $heap->extract();
            $stops++;
        }
        $heap->insert($litres);
    }
    return $stops;
}
