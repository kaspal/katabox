def min_speed(pages: list[int], hours: int) -> int:
    # Correct but O(n·answer): try speeds 1, 2, 3, ... one by one.
    speed = 1
    while True:
        need = 0
        for p in pages:
            need += (p + speed - 1) // speed
        if need <= hours:
            return speed
        speed += 1
