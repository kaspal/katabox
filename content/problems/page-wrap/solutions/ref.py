def wrap_lines(words: list[str], width: int) -> list[str]:
    lines = []
    current = [words[0]]
    length = len(words[0])
    for w in words[1:]:
        if length + 1 + len(w) <= width:
            current.append(w)
            length += 1 + len(w)
        else:
            lines.append(" ".join(current))
            current = [w]
            length = len(w)
    lines.append(" ".join(current))
    return lines
