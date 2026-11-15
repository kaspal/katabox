"""Contact lines in plain Python."""


def solve(tables):
    out = []
    for cid, first, last, phone, email, prefers in tables["clients"]:
        order = [email, phone] if prefers == "email" else [phone, email]
        contact = next((c for c in order if c is not None), "no contact")
        out.append((last, first, cid, f"{last}, {first}: {contact}"))
    out.sort()
    return {"columns": ["id", "contact_line"], "rows": [[cid, line] for _, _, cid, line in out]}
