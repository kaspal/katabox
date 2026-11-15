# Correct but slow: free heights row by row, then every left edge and width, O(R·C²).
import sys


def main() -> None:
    lines = sys.stdin.read().split()
    r, c = int(lines[0]), int(lines[1])
    height = [0] * c
    best = 0
    for row in lines[2 : 2 + r]:
        height = [h + 1 if ch == "." else 0 for h, ch in zip(height, row)]
        for left in range(c):
            low = height[left]
            right = left
            while right < c and low > 0:
                if height[right] < low:
                    low = height[right]
                if low * (right - left + 1) > best:
                    best = low * (right - left + 1)
                right += 1
    print(best)


main()
