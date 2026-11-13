# Obviously correct: read line by line and compare against the exact average as a fraction.
from fractions import Fraction


def main() -> None:
    n = int(input())
    roster = []
    for _ in range(n):
        name, hours = input().split()
        roster.append((name, int(hours)))
    total = sum(hours for _, hours in roster)
    average = Fraction(total, n)
    above = [f"{name} {hours}" for name, hours in roster if hours > average]
    print(total)
    print("\n".join(above) if above else "none")


main()
