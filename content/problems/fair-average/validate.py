import re

GAME = re.compile(r"[A-Z][a-z]{2,11}")


def validate(inp):
    reviews = inp["tables"]["reviews"]
    if not 1 <= len(reviews) <= 3000:
        return "1 ≤ reviews ≤ 3000"
    ids = set()
    for rid, game, stars in reviews:
        if not 1 <= rid <= 10**6 or rid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(rid)
        if not GAME.fullmatch(game):
            return "a game is one capitalised word"
        if stars is not None and stars not in (1, 2, 3, 4, 5):
            return "stars is 1 to 5, or NULL"
    return None
