def shortest_walk(stalls: str, need: str) -> int:
    # Correct but O(n²): grow a stretch from every start until it covers the list.
    want = [0] * 26
    for c in need:
        want[ord(c) - 97] += 1
    codes = [ord(c) - 97 for c in stalls]
    best = -1
    for i in range(len(codes)):
        have, missing = [0] * 26, len(need)
        for j in range(i, len(codes)):
            c = codes[j]
            if have[c] < want[c]:
                missing -= 1
            have[c] += 1
            if missing == 0:
                if best < 0 or j - i + 1 < best:
                    best = j - i + 1
                break
    return best
