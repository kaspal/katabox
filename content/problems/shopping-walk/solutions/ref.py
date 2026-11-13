def shortest_walk(stalls: str, need: str) -> int:
    want = [0] * 26
    for c in need:
        want[ord(c) - 97] += 1
    have = [0] * 26
    missing, left, best = len(need), 0, -1
    codes = [ord(c) - 97 for c in stalls]
    for right, c in enumerate(codes):
        if have[c] < want[c]:
            missing -= 1
        have[c] += 1
        while missing == 0:
            if best < 0 or right - left + 1 < best:
                best = right - left + 1
            d = codes[left]
            have[d] -= 1
            if have[d] < want[d]:
                missing += 1
            left += 1
    return best
