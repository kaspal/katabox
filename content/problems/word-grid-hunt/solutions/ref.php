<?php
function findWords(array $grid, array $words): array {
    $trie = [[]];
    $end = [null];
    foreach ($words as $w) {
        $node = 0;
        $n = strlen($w);
        for ($i = 0; $i < $n; $i++) {
            $ch = $w[$i];
            if (!isset($trie[$node][$ch])) {
                $trie[$node][$ch] = count($trie);
                $trie[] = [];
                $end[] = null;
            }
            $node = $trie[$node][$ch];
        }
        $end[$node] = $w;
    }
    $rows = count($grid);
    $cols = strlen($grid[0]);
    $used = array_fill(0, $rows, array_fill(0, $cols, false));
    $found = [];
    $dfs = function (int $r, int $c, int $node) use (&$dfs, &$used, &$found, $trie, $end, $grid, $rows, $cols): void {
        $ch = $grid[$r][$c];
        if (!isset($trie[$node][$ch])) {
            return;
        }
        $child = $trie[$node][$ch];
        if ($end[$child] !== null) {
            $found[$end[$child]] = true;
        }
        $used[$r][$c] = true;
        foreach ([[$r + 1, $c], [$r - 1, $c], [$r, $c + 1], [$r, $c - 1]] as [$nr, $nc]) {
            if ($nr >= 0 && $nr < $rows && $nc >= 0 && $nc < $cols && !$used[$nr][$nc]) {
                $dfs($nr, $nc, $child);
            }
        }
        $used[$r][$c] = false;
    };
    for ($r = 0; $r < $rows; $r++) {
        for ($c = 0; $c < $cols; $c++) {
            $dfs($r, $c, 0);
        }
    }
    $out = array_map('strval', array_keys($found));
    sort($out, SORT_STRING);
    return $out;
}
