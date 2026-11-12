def wrap_lines(words: list[str], width: int) -> list[str]:
    # Obviously correct: try to extend the last line with each word; otherwise start one.
    lines = []
    for w in words:
        if lines and len(lines[-1] + " " + w) <= width:
            lines[-1] = lines[-1] + " " + w
        else:
            lines.append(w)
    return lines
