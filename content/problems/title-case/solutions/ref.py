def title_case(text: str) -> str:
    out = []
    at_start = True
    for c in text:
        if c == " ":
            out.append(c)
            at_start = True
        else:
            out.append(c.upper() if at_start else c.lower())
            at_start = False
    return "".join(out)
