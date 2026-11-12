def quiet_seats(rows: list[str]) -> int:
    h, w = len(rows), len(rows[0])
    count = 0
    for r in range(h):
        lo_r, hi_r = max(r - 1, 0), min(r + 2, h)
        row = rows[r]
        for c in range(w):
            if row[c] != ".":
                continue
            lo_c, hi_c = max(c - 1, 0), min(c + 2, w)
            if all("#" not in rows[i][lo_c:hi_c] for i in range(lo_r, hi_r)):
                count += 1
    return count
