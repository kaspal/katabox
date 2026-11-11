def compress(text: str) -> str:
    out: list[str] = []
    start = 0
    n = len(text)
    for i in range(1, n + 1):
        if i == n or text[i] != text[start]:
            out.append(text[start])
            if i - start > 1:
                out.append(str(i - start))
            start = i
    return "".join(out)
