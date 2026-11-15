<?php
$lines = explode("\n", stream_get_contents(STDIN));
$q = (int) $lines[0];
$balance = [];
$holders = []; // balance → how many accounts hold it (for COUNT)
// One undo log for all open transactions: [name, balance before or null]. BEGIN remembers
// where the log stood; ROLLBACK undoes back to there; COMMIT only forgets the mark, so its
// changes now belong to the enclosing transaction. Names are letters: safe array keys.
$log = [];
$marks = [];

function put(array &$balance, array &$holders, string $name, ?int $value): void
{
    if (isset($balance[$name])) {
        $holders[$balance[$name]]--;
    }
    if ($value === null) {
        unset($balance[$name]);
    } else {
        $balance[$name] = $value;
        $holders[$value] = ($holders[$value] ?? 0) + 1;
    }
}

$out = [];
for ($i = 1; $i <= $q; $i++) {
    $parts = explode(' ', trim($lines[$i]));
    switch ($parts[0]) {
        case 'SET':
        case 'ADD':
            $name = $parts[1];
            $value = $parts[0] === 'SET' ? (int) $parts[2] : ($balance[$name] ?? 0) + (int) $parts[2];
            if ($marks) {
                $log[] = [$name, $balance[$name] ?? null];
            }
            put($balance, $holders, $name, $value);
            break;
        case 'GET':
            $out[] = $balance[$parts[1]] ?? 'NONE';
            break;
        case 'COUNT':
            $out[] = $holders[(int) $parts[1]] ?? 0;
            break;
        case 'BEGIN':
            $marks[] = count($log);
            break;
        default: // ROLLBACK or COMMIT
            if (!$marks) {
                $out[] = 'NO TRANSACTION';
            } elseif ($parts[0] === 'ROLLBACK') {
                $mark = array_pop($marks);
                while (count($log) > $mark) {
                    [$name, $old] = array_pop($log);
                    put($balance, $holders, $name, $old);
                }
            } else {
                array_pop($marks);
                if (!$marks) {
                    $log = []; // nothing can be rolled back any more
                }
            }
    }
}
echo $out ? implode("\n", $out) . "\n" : '';
