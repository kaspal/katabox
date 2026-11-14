# Obviously correct: try every way to cut the digits into pieces (each gap cut or not)
# and keep the cuttings whose pieces are all letter numbers "1" … "26".
MOD = 1_000_003
CODES = {str(v) for v in range(1, 27)}


def count(digits: str) -> int:
    n = len(digits)
    total = 0
    for mask in range(1 << (n - 1)):
        pieces = []
        start = 0
        for gap in range(n - 1):
            if mask >> gap & 1:
                pieces.append(digits[start : gap + 1])
                start = gap + 1
        pieces.append(digits[start:])
        if all(p in CODES for p in pieces):
            total += 1
    return total % MOD


def main() -> None:
    t = int(input())
    for _ in range(t):
        line = input().strip()
        assert line.startswith("#")
        print(count(line[1:]))


main()
