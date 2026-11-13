def road_repairs(length: int, repairs: list[list[int]]) -> list[int]:
    # Correct but O(length·repairs): add every repair section by section.
    road = [0] * length
    for a, b, crews in repairs:
        for i in range(a, b + 1):
            road[i] += crews
    return road
