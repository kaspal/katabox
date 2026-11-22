def race_order(names: list[str], times: list[int]) -> list[str]:
    order = sorted(range(len(names)), key=lambda i: names[i], reverse=True)
    order.sort(key=lambda i: times[i])
    return [names[i] for i in order]
