def longest_balanced_part(text: str) -> int:
    # Obviously correct: test every part.
    pairs = {")": "(", "]": "[", "}": "{"}

    def balanced(s: str) -> bool:
        st = []
        for c in s:
            if c in "([{":
                st.append(c)
            elif not st or st.pop() != pairs[c]:
                return False
        return not st

    best = 0
    for i in range(len(text)):
        for j in range(i + 1, len(text) + 1):
            if balanced(text[i:j]):
                best = max(best, j - i)
    return best
