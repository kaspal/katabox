<?php
function smallestSquad(int $skills, array $people): int {
    $full = (1 << $skills) - 1;
    $moves = array_values(array_unique($people));
    $dist = array_fill(0, $full + 1, -1);
    $dist[0] = 0;
    $queue = [0];
    for ($head = 0; $head < count($queue); $head++) {
        $mask = $queue[$head];
        if ($mask === $full) {
            return $dist[$mask];
        }
        foreach ($moves as $p) {
            $m = $mask | $p;
            if ($dist[$m] < 0) {
                $dist[$m] = $dist[$mask] + 1;
                $queue[] = $m;
            }
        }
    }
    return -1;
}
