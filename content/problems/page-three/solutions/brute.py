"""Page three of the visible listings, in plain Python."""


def solve(tables):
    visible = [(-rating, title, lid) for lid, title, rating, hidden in tables["listings"] if hidden == 0]
    visible.sort()
    page = visible[20:30]
    return {"columns": ["id", "title", "rating"], "rows": [[lid, title, -neg] for neg, title, lid in page]}
