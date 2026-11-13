def road_repairs(length: int, repairs: list[list[int]]) -> list[int]:
    change = [0] * (length + 1)
    for a, b, crews in repairs:
        change[a] += crews
        change[b + 1] -= crews
    out, running = [], 0
    for i in range(length):
        running += change[i]
        out.append(running)
    return out
