def longest_balanced_part(text: str) -> int:
    pairs = {")": "(", "]": "[", "}": "{"}
    stack = [-1]  # positions; the bottom is the last position no balanced part can cross
    best = 0
    for i, c in enumerate(text):
        if c in "([{":
            stack.append(i)
        elif len(stack) > 1 and text[stack[-1]] == pairs[c]:
            stack.pop()
            best = max(best, i - stack[-1])
        else:
            stack = [i]
    return best
