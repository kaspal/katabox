<?php
function seatMap(int $rows, int $cols, array $bookings): array {
    $hall = array_fill(0, $rows, str_repeat('.', $cols));
    foreach ($bookings as [$r, $c]) {
        $hall[$r][$c] = 'X';
    }
    return $hall;
}
