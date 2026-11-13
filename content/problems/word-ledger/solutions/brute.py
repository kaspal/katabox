# Obviously correct: build words character by character, then pick the best word K times.
import sys


def main() -> None:
    first, _, text = sys.stdin.read().partition("\n")
    k = int(first)
    counts: dict[str, int] = {}
    word = ""
    for ch in text + " ":
        if "a" <= ch <= "z":
            word += ch
        else:
            if word:
                counts[word] = counts.get(word, 0) + 1
            word = ""
    out = []
    while counts and len(out) < k:
        best = None
        for w, c in counts.items():
            if best is None or c > counts[best] or (c == counts[best] and w < best):
                best = w
        out.append(f"{best} {counts.pop(best)}")
    print("\n".join(out))


main()
