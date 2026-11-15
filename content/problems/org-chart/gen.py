"""Company charts and "is a above b?" questions as stdin text (a program problem, ADR 0028)."""
import random


def chart(rng, n, spine):
    """boss[i] for employees 2..n (index 0 and 1 unused): a line of `spine` managers below
    the director, the others hung on random earlier employees; numbers shuffled except the
    director's."""
    order = [1] + rng.sample(range(2, n + 1), n - 1)
    boss = [0] * (n + 1)
    for k in range(1, n):
        parent = k - 1 if k < spine else rng.randrange(k)
        boss[order[k]] = order[parent]
    return boss, order


def chart_text(boss):
    n = len(boss) - 1
    return " ".join(str(boss[i]) for i in range(2, n + 1))


def render_chart(boss, questions):
    n = len(boss) - 1
    lines = [str(n), chart_text(boss), str(len(questions))]
    lines += [f"{a} {b}" for a, b in questions]
    return "\n".join(lines) + "\n"


def mixed_questions(rng, n, order, boss, q):
    """Half ask about someone and an employee below them, half are random pairs."""
    questions = []
    for _ in range(q):
        if rng.random() < 0.5 and n > 1:
            b = rng.randint(2, n)
            a = b
            for _ in range(rng.randint(1, 50)):
                if a == 1:
                    break
                a = boss[a]
            questions.append((a, b))
        else:
            questions.append((rng.randint(1, n), rng.randint(1, n)))
    return questions


def sized(rng, n):
    """Complexity profile input (ADR 0024): a chart with a spine of n // 2 managers and
    n // 5 questions about the bottom of the spine."""
    boss, order = chart(rng, n, n // 2)
    spine = order[: n // 2]
    questions = [(rng.choice(spine[:10]), rng.choice(spine[-10:])) for _ in range(max(1, n // 5))]
    return render_chart(boss, questions)


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # A line of 10⁵ employees, each the boss of the next (deep: recursion fails),
            # and 2·10⁴ questions about the top and the bottom of the line: walking up from
            # b is about 10⁹ steps. Numbered in order, so the input stays small.
            n = 100_000
            boss = [0, 0] + list(range(1, n))
            questions = []
            for _ in range(20_000):
                a, b = rng.randint(1, 50), rng.randint(n - 50, n)
                questions.append((b, a) if rng.random() < 0.3 else (a, b))
            return render_chart(boss, questions)
        # A random chart of 2·10⁴ with shuffled numbers (a boss may have a larger number).
        n = 20_000
        boss, order = chart(rng, n, 200)
        return render_chart(boss, mixed_questions(rng, n, order, boss, 5_000))
    if seed % 5 == 0:
        # Edge shapes: the director alone, a == b, the director as b, a two-person chart.
        kind = rng.randrange(3)
        if kind == 0:
            return render_chart([0, 0], [(1, 1)])
        if kind == 1:
            return render_chart([0, 0, 1], [(1, 2), (2, 1), (2, 2), (1, 1)])
        boss, order = chart(rng, 6, 6)
        return render_chart(boss, [(a, b) for a in (1, order[5]) for b in (1, order[5])])
    n = rng.randint(1, 9)
    boss, order = chart(rng, n, rng.randint(1, n))
    return render_chart(boss, mixed_questions(rng, n, order, boss, rng.randint(1, 8)))


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
