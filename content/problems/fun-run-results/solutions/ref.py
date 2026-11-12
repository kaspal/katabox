def finish_order(names: list[str], times: list[int]) -> list[str]:
    order = sorted(range(len(names)), key=lambda i: (times[i], names[i]))
    return [names[i] for i in order]
