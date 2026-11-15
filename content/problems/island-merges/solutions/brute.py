# Obviously correct: after every new square, count the islands from scratch with a flood
# fill over the whole map.
def main() -> None:
    r, c, q = map(int, input().split())
    land = [[False] * (c + 2) for _ in range(r + 2)]
    for _ in range(q):
        y, x = map(int, input().split())
        land[y][x] = True
        seen = [[False] * (c + 2) for _ in range(r + 2)]
        islands = 0
        for sy in range(1, r + 1):
            for sx in range(1, c + 1):
                if land[sy][sx] and not seen[sy][sx]:
                    islands += 1
                    stack = [(sy, sx)]
                    seen[sy][sx] = True
                    while stack:
                        cy, cx = stack.pop()
                        for ny, nx in ((cy + 1, cx), (cy - 1, cx), (cy, cx + 1), (cy, cx - 1)):
                            if land[ny][nx] and not seen[ny][nx]:
                                seen[ny][nx] = True
                                stack.append((ny, nx))
        print(islands)


main()
