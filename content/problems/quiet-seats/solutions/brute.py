def quiet_seats(rows: list[str]) -> int:
    # Obviously correct: for each free seat, test all 8 directions one by one.
    h, w = len(rows), len(rows[0])
    count = 0
    for r in range(h):
        for c in range(w):
            if rows[r][c] != ".":
                continue
            ok = True
            for dr in (-1, 0, 1):
                for dc in (-1, 0, 1):
                    if dr == 0 and dc == 0:
                        continue
                    i, j = r + dr, c + dc
                    if 0 <= i < h and 0 <= j < w and rows[i][j] == "#":
                        ok = False
            if ok:
                count += 1
    return count
