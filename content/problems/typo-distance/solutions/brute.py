def typo_distance(typed: str, meant: str) -> int:
    # Obviously correct (for short words): breadth-first search over every string
    # reachable with one fix at a time, using only letters that appear in the words.
    letters = sorted(set(typed + meant))
    limit = len(typed) + len(meant)
    frontier, seen, steps = {typed}, {typed}, 0
    while frontier:
        if meant in frontier:
            return steps
        nxt = set()
        for s in frontier:
            options = set()
            for i in range(len(s) + 1):
                for c in letters:
                    options.add(s[:i] + c + s[i:])
            for i in range(len(s)):
                options.add(s[:i] + s[i + 1:])
                for c in letters:
                    options.add(s[:i] + c + s[i + 1:])
            for t in options:
                if t not in seen and len(t) <= limit:
                    seen.add(t)
                    nxt.add(t)
        frontier, steps = nxt, steps + 1
    return -1
