def live_counts(ops: list[str], words: list[str]) -> list[int]:
    # A trie of dicts; count[node] = titles passing through that node.
    children: list[dict[str, int]] = [{}]
    count = [0]
    out = []
    for op, w in zip(ops, words):
        if op == "count":
            node = 0
            for c in w:
                node = children[node].get(c, -1)
                if node == -1:
                    break
            out.append(count[node] if node != -1 else 0)
            continue
        step = 1 if op == "add" else -1
        node = 0
        for c in w:
            nxt = children[node].get(c)
            if nxt is None:
                children.append({})
                count.append(0)
                nxt = len(count) - 1
                children[node][c] = nxt
            node = nxt
            count[node] += step
    return out
