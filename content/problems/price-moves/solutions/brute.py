"""Day-on-day changes per ticker, in plain Python."""
from decimal import ROUND_HALF_UP, Decimal


def solve(tables):
    by_ticker = {}
    for _, ticker, day, close in tables["prices"]:
        by_ticker.setdefault(ticker, []).append((day, close))
    rows = []
    for ticker in sorted(by_ticker):
        series = sorted(by_ticker[ticker])
        for (_, prev), (day, close) in zip(series, series[1:]):
            # As SQLite: a real division, then round() half away from zero.
            pct = float(Decimal(100.0 * (close - prev) / prev).quantize(Decimal("0.01"), ROUND_HALF_UP))
            rows.append([ticker, day, close, prev, pct])
    return {"columns": ["ticker", "day", "close_cents", "prev_cents", "change_pct"], "rows": rows}
