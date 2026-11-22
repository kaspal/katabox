def live_counts(ops: list[str], words: list[str]) -> list[int]:
    # Obviously correct: keep the titles in a list and count matches each time.
    titles: list[str] = []
    out = []
    for op, w in zip(ops, words):
        if op == "add":
            titles.append(w)
        elif op == "remove":
            titles.remove(w)
        else:
            out.append(sum(1 for t in titles if t.startswith(w)))
    return out
