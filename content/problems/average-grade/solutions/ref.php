<?php
function averageGrade(array $grades): float {
    return array_sum($grades) / count($grades);
}
