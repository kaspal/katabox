<?php
function notebook(array $ops, array $args): array {
    $values = [];
    $undo = [];
    $redo = [];
    $reports = [];
    foreach ($ops as $i => $op) {
        $a = $args[$i];
        if ($op === 'write') {
            $undo[] = [$a[0], $values[$a[0]] ?? null, $a[1]];
            $values[$a[0]] = $a[1];
            $redo = [];
        } elseif ($op === 'read') {
            $reports[] = $values[$a[0]] ?? -1;
        } elseif ($op === 'undo') {
            if ($undo) {
                $c = array_pop($undo);
                if ($c[1] === null) {
                    unset($values[$c[0]]);
                } else {
                    $values[$c[0]] = $c[1];
                }
                $redo[] = $c;
            }
        } else {
            if ($redo) {
                $c = array_pop($redo);
                $values[$c[0]] = $c[2];
                $undo[] = $c;
            }
        }
    }
    return $reports;
}
