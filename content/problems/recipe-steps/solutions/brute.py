def step_order(n: int, before: list[list[int]]) -> list[int]:
    # Obviously correct: like the cook, repeatedly do the lowest-numbered step whose
    # every required step is already done.
    done = []
    finished = set()
    while len(done) < n:
        for s in range(n):
            if s not in finished and all(a in finished for a, b in before if b == s):
                done.append(s)
                finished.add(s)
                break
        else:
            return []
    return done
