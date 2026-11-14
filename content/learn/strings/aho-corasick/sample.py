from collections import deque


def aho_corasick(text, patterns):
    goto, fail, out = [{}], [0], [[]]
    for p, word in enumerate(patterns):
        node = 0
        for ch in word:
            if ch not in goto[node]:
                goto.append({})
                fail.append(0)
                out.append([])
                goto[node][ch] = len(goto) - 1
            node = goto[node][ch]
        out[node].append(p)
    queue = deque(goto[0].values())
    while queue:
        node = queue.popleft()
        for ch, child in goto[node].items():
            f = fail[node]
            while f and ch not in goto[f]:
                f = fail[f]
            fail[child] = goto[f].get(ch, 0)  # @step link
            out[child] = out[child] + out[fail[child]]
            queue.append(child)
    counts, node = [0] * len(patterns), 0
    for ch in text:
        while node and ch not in goto[node]:
            node = fail[node]  # @step follow
        node = goto[node].get(ch, 0)
        for p in out[node]:
            counts[p] += 1  # @step hit
    return counts
