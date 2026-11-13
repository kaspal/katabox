def largest_poster(panels: list[int]) -> int:
    stack, best = [], 0
    for i, h in enumerate(panels + [0]):
        while stack and panels[stack[-1]] > h:
            k = stack.pop()
            width = i - stack[-1] - 1 if stack else i
            best = max(best, panels[k] * width)
        stack.append(i)
    return best
