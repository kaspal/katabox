import re
from datetime import date

TICKER = re.compile(r"[A-Z]{3,4}")


def validate(inp):
    prices = inp["tables"]["prices"]
    if len(prices) > 3000:
        return "at most 3000 prices"
    ids, seen = set(), set()
    for pid, ticker, day, close in prices:
        if not 1 <= pid <= 10**6 or pid in ids:
            return "ids are distinct, 1 to 10⁶"
        ids.add(pid)
        if not TICKER.fullmatch(ticker):
            return "a ticker is 3-4 capital letters"
        try:
            ok = re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", day) and date.fromisoformat(day).year == 2026
        except ValueError:
            ok = False
        if not ok:
            return "day is a date of 2026"
        if (ticker, day) in seen:
            return "one close per ticker and day"
        seen.add((ticker, day))
        if not 1 <= close <= 10**7:
            return "1 ≤ close_cents ≤ 10⁷"
    return None
