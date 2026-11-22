def race_order(names: list[str], times: list[int]) -> list[str]:
    return [n for _, n in sorted(zip(times, names))]
