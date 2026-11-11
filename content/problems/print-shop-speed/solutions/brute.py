def min_speed(pages: list[int], hours: int) -> int:
    # Obviously correct: try speeds in order.
    speed = 1
    while sum(-(-p // speed) for p in pages) > hours:
        speed += 1
    return speed
