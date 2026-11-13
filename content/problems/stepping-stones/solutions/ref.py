def can_cross(jumps: list[int]) -> bool:
    reach = 0
    for i, j in enumerate(jumps):
        if i > reach:
            return False
        if i + j > reach:
            reach = i + j
    return True
