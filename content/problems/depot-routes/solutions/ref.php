<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$n = (int) $tokens[$pos++];
$m = (int) $tokens[$pos++];
// adj[town] holds flat pairs: neighbour, minutes, neighbour, minutes, ...
$adj = array_fill(0, $n + 1, []);
for ($i = 0; $i < $m; $i++) {
    $a = (int) $tokens[$pos++];
    $b = (int) $tokens[$pos++];
    $w = (int) $tokens[$pos++];
    $adj[$a][] = $b;
    $adj[$a][] = $w;
    $adj[$b][] = $a;
    $adj[$b][] = $w;
}
$inf = PHP_INT_MAX;
$dist = array_fill(0, $n + 1, $inf);
$dist[1] = 0;
$heap = new SplPriorityQueue();
$heap->setExtractFlags(SplPriorityQueue::EXTR_BOTH);
$heap->insert(1, 0); // priorities are negated times (SplPriorityQueue is a max-heap)
while (!$heap->isEmpty()) {
    $top = $heap->extract();
    $u = $top['data'];
    $d = -$top['priority'];
    if ($d > $dist[$u]) {
        continue; // a stale entry
    }
    $edges = $adj[$u];
    $len = count($edges);
    for ($k = 0; $k < $len; $k += 2) {
        $v = $edges[$k];
        $nd = $d + $edges[$k + 1];
        if ($nd < $dist[$v]) {
            $dist[$v] = $nd;
            $heap->insert($v, -$nd);
        }
    }
}
$q = (int) $tokens[$pos++];
$out = [];
for ($i = 0; $i < $q; $i++) {
    $d = $dist[(int) $tokens[$pos++]];
    $out[] = $d === $inf ? -1 : $d;
}
echo implode("\n", $out), "\n";
