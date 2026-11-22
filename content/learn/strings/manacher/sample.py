def manacher(text):
    s = "|" + "|".join(text) + "|"
    n = len(s)
    radius = [0] * n
    center = right = 0
    for i in range(n):
        if i < right:
            radius[i] = min(right - i, radius[2 * center - i])  # @step mirror
        while (
            i - radius[i] - 1 >= 0
            and i + radius[i] + 1 < n
            and s[i - radius[i] - 1] == s[i + radius[i] + 1]
        ):
            radius[i] += 1  # @step grow
        if i + radius[i] > right:
            center, right = i, i + radius[i]
    best = 0
    for i in range(n):
        if radius[i] > radius[best]:
            best = i
    return [radius[best], (best - radius[best]) // 2]
