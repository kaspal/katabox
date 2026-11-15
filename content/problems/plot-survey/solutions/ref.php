<?php
function gcd(int $a, int $b): int
{
    while ($b !== 0) {
        [$a, $b] = [$b, $a % $b];
    }
    return $a;
}

$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$xs = [];
$ys = [];
for ($i = 0; $i < $n; $i++) {
    $xs[] = (int) $tokens[1 + 2 * $i];
    $ys[] = (int) $tokens[2 + 2 * $i];
}
$twice = 0; // twice the area: the shoelace formula, in whole numbers
$boundary = 0; // lattice points on the edges: an edge (dx, dy) holds gcd(|dx|, |dy|)
for ($i = 0; $i < $n; $i++) {
    $j = ($i + 1) % $n;
    $twice += $xs[$i] * $ys[$j] - $xs[$j] * $ys[$i];
    $boundary += gcd(abs($xs[$j] - $xs[$i]), abs($ys[$j] - $ys[$i]));
}
$twice = abs($twice);
// Pick's theorem: area = inside + boundary / 2 - 1, so inside = (2·area - boundary + 2) / 2.
$inside = intdiv($twice - $boundary + 2, 2);
echo intdiv($twice, 2), $twice % 2 ? '.5' : '.0', ' ', $inside, "\n";
