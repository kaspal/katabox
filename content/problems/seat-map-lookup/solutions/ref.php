<?php
function hasTicket(array $seats, int $ticket): bool {
    $r = 0;
    $c = count($seats[0]) - 1;
    $rows = count($seats);
    while ($r < $rows && $c >= 0) {
        $v = $seats[$r][$c];
        if ($v === $ticket) {
            return true;
        }
        if ($v > $ticket) {
            $c--;
        } else {
            $r++;
        }
    }
    return false;
}
