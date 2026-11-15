<?php
function minutes(string $t): int
{
    return (int) substr($t, 0, 2) * 60 + (int) substr($t, 3);
}

function clock(int $m): string
{
    return sprintf('%02d:%02d', intdiv($m, 60), $m % 60);
}

$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$t = (int) $tokens[$pos++];
$out = [];
for ($d = 0; $d < $t; $d++) {
    $open = minutes($tokens[$pos++]);
    $close = minutes($tokens[$pos++]);
    $k = (int) $tokens[$pos++];
    $slots = [];
    for ($i = 0; $i < $k; $i++) {
        $slots[] = [minutes($tokens[$pos++]), minutes($tokens[$pos++])];
    }
    usort($slots, fn($a, $b) => $a[0] <=> $b[0]);
    // Sweep the slots by start time; $freeFrom is where the free time begins. A gap is
    // only replaced by a strictly longer one, so the earliest longest gap wins.
    $bestLen = 0;
    $bestStart = 0;
    $freeFrom = $open;
    foreach ($slots as [$s, $e]) {
        if ($s - $freeFrom > $bestLen) {
            $bestLen = $s - $freeFrom;
            $bestStart = $freeFrom;
        }
        $freeFrom = max($freeFrom, $e);
    }
    if ($close - $freeFrom > $bestLen) {
        $bestLen = $close - $freeFrom;
        $bestStart = $freeFrom;
    }
    $out[] = $bestLen > 0 ? clock($bestStart) . '-' . clock($bestStart + $bestLen) : 'BOOKED';
}
echo implode("\n", $out), "\n";
