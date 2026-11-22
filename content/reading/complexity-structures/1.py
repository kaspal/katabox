def f(n):
    seen = set()
    repeats = 0
    for i in range(n):
        key = i * i % n
        if key in seen:
            repeats += 1
        else:
            seen.add(key)
    return repeats
