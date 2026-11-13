<?php
// Iterative BFS: returns [the last junction reached, its distance], a farthest one.
function farthest(array $adj, int $n, int $from): array
{
    $dist = array_fill(0, $n + 1, -1);
    $dist[$from] = 0;
    $queue = [$from];
    for ($head = 0; $head < count($queue); $head++) {
        $v = $queue[$head];
        $d = $dist[$v] + 1;
        foreach ($adj[$v] as $w) {
            if ($dist[$w] < 0) {
                $dist[$w] = $d;
                $queue[] = $w;
            }
        }
    }
    $last = $queue[count($queue) - 1];
    return [$last, $dist[$last]];
}

$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$t = (int) $tokens[$pos++];
$out = [];
for ($c = 0; $c < $t; $c++) {
    $n = (int) $tokens[$pos++];
    $adj = array_fill(0, $n + 1, []);
    for ($i = 0; $i < $n - 1; $i++) {
        $a = (int) $tokens[$pos++];
        $b = (int) $tokens[$pos++];
        $adj[$a][] = $b;
        $adj[$b][] = $a;
    }
    [$u] = farthest($adj, $n, 1);
    $out[] = farthest($adj, $n, $u)[1];
}
echo implode("\n", $out), "\n";
