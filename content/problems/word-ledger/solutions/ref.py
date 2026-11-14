import re
import sys
from collections import Counter


def main() -> None:
    first, _, text = sys.stdin.buffer.read().partition(b"\n")
    k = int(first)
    counts = Counter(re.findall(rb"[a-z]+", text))
    ranked = sorted(counts.items(), key=lambda item: (-item[1], item[0]))[:k]
    sys.stdout.write("".join(f"{word.decode()} {count}\n" for word, count in ranked))


main()
