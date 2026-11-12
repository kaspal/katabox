def rotation_groups(codes: list[str]) -> int:
    # Correct but O(n²·L): compare each code with every earlier code it might repeat.
    n = len(codes)
    count = 0
    for i in range(n):
        a = codes[i]
        new = True
        for j in range(i):
            b = codes[j]
            if len(a) == len(b) and a in b + b:
                new = False
                break
        if new:
            count += 1
    return count
