def fewest_shelves(widths: list[int], width: int) -> int:
    books = sorted(widths, reverse=True)
    lower = -(-sum(books) // width)

    # First fit decreasing: a good first answer.
    loads: list[int] = []
    for b in books:
        for i, load in enumerate(loads):
            if load + b <= width:
                loads[i] += b
                break
        else:
            loads.append(b)
    best = len(loads)

    def place(i: int, loads: list[int]) -> None:
        nonlocal best
        if best == lower:
            return
        if i == len(books):
            best = min(best, len(loads))
            return
        b = books[i]
        tried = set()
        for k in range(len(loads)):
            if loads[k] + b <= width and loads[k] not in tried:
                tried.add(loads[k])
                loads[k] += b
                place(i + 1, loads)
                loads[k] -= b
        if len(loads) + 1 < best:
            loads.append(b)
            place(i + 1, loads)
            loads.pop()

    place(0, [])
    return best
