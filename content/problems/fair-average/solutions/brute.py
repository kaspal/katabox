"""Rated reviews and their average per game, in plain Python."""
from decimal import ROUND_HALF_UP, Decimal


def solve(tables):
    games = {}
    for _, game, stars in tables["reviews"]:
        games.setdefault(game, [])
        if stars is not None:
            games[game].append(stars)
    rows = []
    for game in sorted(games):
        s = games[game]
        # SQLite's round(): half away from zero, on the average's exact binary value.
        avg = None if not s else float(Decimal(sum(s) / len(s)).quantize(Decimal("0.01"), ROUND_HALF_UP))
        rows.append([game, len(s), avg])
    return {"columns": ["game", "rated", "avg_stars"], "rows": rows}
