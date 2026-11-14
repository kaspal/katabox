import re

# Obviously correct: split each label into (count, letter) runs, and for each check
# skip whole runs, taking their length off k, until k falls inside one.


def main() -> None:
    n_labels = int(input())
    labels = []
    for _ in range(n_labels):
        labels.append([(int(c), ch) for c, ch in re.findall(r"(\d+)([a-z])", input().strip())])
    q = int(input())
    for _ in range(q):
        i, k = map(int, input().split())
        for count, ch in labels[i - 1]:
            if k <= count:
                print(ch)
                break
            k -= count


main()
