<?php
function seconds(string $lap): int
{
    [$m, $s] = explode(':', $lap);
    return (int) $m * 60 + (int) $s;
}

function clock(int $total): string
{
    return intdiv($total, 60) . ':' . str_pad((string) ($total % 60), 2, '0', STR_PAD_LEFT);
}

$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$t = (int) $tokens[$pos++];
$out = [];
for ($i = 0; $i < $t; $i++) {
    $name = $tokens[$pos++];
    $k = (int) $tokens[$pos++];
    $total = 0;
    $best = PHP_INT_MAX;
    for ($j = 0; $j < $k; $j++) {
        $lap = seconds($tokens[$pos++]);
        $total += $lap;
        $best = min($best, $lap);
    }
    $out[] = $name . ' ' . clock($total) . ' ' . clock($best);
}
echo implode("\n", $out), "\n";
