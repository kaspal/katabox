def word_break(text, words):
    n = len(text)
    ok = [False] * (n + 1)
    ok[0] = True
    for end in range(1, n + 1):
        for word in words:
            start = end - len(word)
            if start >= 0 and ok[start] and text[start:end] == word:
                ok[end] = True  # @step fits
                break
    return [1 if ok[n] else 0]
