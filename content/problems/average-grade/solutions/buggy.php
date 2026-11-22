<?php
function averageGrade(array $grades): float {
    return intdiv(array_sum($grades), count($grades));
}
