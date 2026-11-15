# Obviously correct: place both hands in degrees from 12 o'clock and compare them.
def main() -> None:
    q = int(input())
    for _ in range(q):
        h, m = map(int, input().split(":"))
        hour_hand = (h % 12) * 30 + m * 0.5  # 360° in 12 hours, moving every minute
        minute_hand = m * 6  # 360° in 60 minutes
        gap = abs(hour_hand - minute_hand)
        print(f"{min(gap, 360 - gap):.1f}")


main()
