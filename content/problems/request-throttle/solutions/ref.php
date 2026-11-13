<?php
function throttle(int $limit, int $window, array $ops, array $args): array {
    $logs = [];
    $out = [];
    foreach ($ops as $i => $op) {
        [$user, $time] = $args[$i];
        if (!isset($logs[$user])) {
            $logs[$user] = new SplQueue();
        }
        $log = $logs[$user];
        while (!$log->isEmpty() && $log->bottom() <= $time - $window) {
            $log->dequeue();
        }
        if ($op === 'count') {
            $out[] = count($log);
        } elseif (count($log) < $limit) {
            $log->enqueue($time);
            $out[] = 1;
        } else {
            $out[] = 0;
        }
    }
    return $out;
}
