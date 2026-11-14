def kmp(text, pattern):
    m = len(pattern)
    fail = [0] * m
    k = 0
    for i in range(1, m):
        while k > 0 and pattern[i] != pattern[k]:
            k = fail[k - 1]  # @step fall
        if pattern[i] == pattern[k]:
            k += 1
        fail[i] = k  # @step border
    hits, k = [], 0
    for i, ch in enumerate(text):
        while k > 0 and ch != pattern[k]:
            k = fail[k - 1]  # @step skip
        if ch == pattern[k]:
            k += 1
        if k == m:
            hits.append(i - m + 1)  # @step match
            k = fail[k - 1]
    return [len(hits)] + hits
