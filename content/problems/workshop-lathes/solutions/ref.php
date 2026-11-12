<?php
function finishTime(array $jobs, int $lathes): int {
    $free = new SplMinHeap();
    $m = min($lathes, count($jobs));
    for ($i = 0; $i < $m; $i++) {
        $free->insert(0);
    }
    $latest = 0;
    foreach ($jobs as $job) {
        $end = $free->extract() + $job;
        $free->insert($end);
        $latest = max($latest, $end);
    }
    return $latest;
}
