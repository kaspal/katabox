<?php
function assignRooms(array $meetings): array {
    $order = array_keys($meetings);
    usort($order, fn($x, $y) => [$meetings[$x][0], $x] <=> [$meetings[$y][0], $y]);
    $free = new SplMinHeap();
    $busy = new SplMinHeap(); // [end, room]
    $rooms = 0;
    $out = array_fill(0, count($meetings), 0);
    foreach ($order as $i) {
        [$s, $e] = $meetings[$i];
        while (!$busy->isEmpty() && $busy->top()[0] <= $s) {
            $free->insert($busy->extract()[1]);
        }
        if (!$free->isEmpty()) {
            $room = $free->extract();
        } else {
            $rooms++;
            $room = $rooms;
        }
        $out[$i] = $room;
        $busy->insert([$e, $room]);
    }
    return $out;
}
