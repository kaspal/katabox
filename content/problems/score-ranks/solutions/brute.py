# Obviously correct: sort all scores from highest to lowest, walk them handing out
# ranks (a new rank each time the score changes), then look each question up by scanning.
def main() -> None:
    n = int(input())
    scores = [int(x) for x in input().split()]
    assert len(scores) == n
    q = int(input())
    asked = [int(x) for x in input().split()]
    assert len(asked) == q
    ranked = []  # (score, rank) for every student
    rank = 0
    previous = None
    for s in sorted(scores, reverse=True):
        if s != previous:
            rank += 1
            previous = s
        ranked.append((s, rank))
    answers = []
    for x in asked:
        answer = 0
        for s, r in ranked:
            if s == x:
                answer = r
        answers.append(answer)
    print(" ".join(map(str, answers)))


main()
