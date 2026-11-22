def fewest_insertions(text: str) -> int:
    open_ = 0  # unmatched "(" so far
    added = 0  # "(" inserted for ")" with nothing to close
    for c in text:
        if c == "(":
            open_ += 1
        elif c == ")":
            if open_:
                open_ -= 1
            else:
                added += 1
    return added + open_
