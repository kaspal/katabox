"""Bus timetables as stdin text (a program problem, ADR 0028)."""
import math
import random

LIMIT = 9 * 10**15  # the most the lcm of a timetable's periods may be (< 2⁵³)
MAX_PERIOD = 10**6


def lcm(a, b):
    return a // math.gcd(a, b) * b


def lcm_all(values):
    total = 1
    for v in values:
        total = lcm(total, v)
    return total


def pick(rng, current, draw):
    """A period from `draw()` that keeps the timetable's lcm (now `current`) within LIMIT."""
    for _ in range(40):
        m = draw()
        if lcm(current, m) <= LIMIT:
            return m
    return math.gcd(current, rng.randint(1, MAX_PERIOD))  # a divisor of the lcm never raises it


def divisors(v):
    small = [d for d in range(1, math.isqrt(v) + 1) if v % d == 0]
    return small + [v // d for d in small]


def residues(rng, periods, kind, late=False):
    """'meet': the lines share a minute (late: in the second half of the lcm); 'never':
    like meet, then one line shifted by a minute against a line it shares a factor with;
    'random': any first departures."""
    if kind == "random":
        return [rng.randrange(m) for m in periods]
    total = lcm_all(periods)
    t = rng.randrange(total // 2, total) if late else rng.randrange(total)
    rs = [t % m for m in periods]
    if kind == "never":
        clashes = [
            i
            for i in range(len(periods))
            for j in range(len(periods))
            if i != j and math.gcd(periods[i], periods[j]) > 1
        ]
        if clashes:
            i = rng.choice(clashes)
            rs[i] = (rs[i] + 1) % periods[i]
    return rs


def heavy(rng, kind):
    """Three large periods with an lcm of 10¹²..9·10¹⁵, plus lines whose periods divide it.
    Stepping through the departures of the longest period takes over lcm/10⁶ steps."""
    while True:
        if rng.random() < 0.5:
            periods = [rng.randint(50_000, 200_000) for _ in range(3)]
        else:
            periods = [rng.randint(500_000, MAX_PERIOD), rng.randint(500_000, MAX_PERIOD), rng.randint(2, 9_000)]
        if 10**12 <= lcm_all(periods) <= LIMIT:
            break
    for _ in range(rng.randint(0, 3)):
        a, b = rng.sample(periods[:3], 2)
        m = lcm(rng.choice(divisors(a)), rng.choice(divisors(b)))  # divides lcm(a, b)
        periods.append(m if m <= MAX_PERIOD else a)
    rng.shuffle(periods)
    return periods, residues(rng, periods, kind, late=True)


def light(rng):
    n = rng.randint(1, 6)
    style = rng.randrange(3)
    base = rng.randint(2, 1_000)
    draws = [
        lambda: rng.randint(1, 60),
        lambda: rng.randint(1, MAX_PERIOD),
        lambda: base * rng.randint(1, MAX_PERIOD // base),  # periods sharing a factor
    ]
    periods = []
    current = 1
    for _ in range(n):
        m = pick(rng, current, draws[style])
        periods.append(m)
        current = lcm(current, m)
    kind = rng.choice(["meet", "meet", "random", "never"])
    return periods, residues(rng, periods, kind)


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 2000 timetables (about 110 KB); 300 of them have an lcm of 10¹²..9·10¹⁵ and a late
        # meeting minute or none, so stepping through departures is far too slow.
        cases = [heavy(rng, "never" if i % 3 == 0 else "meet") for i in range(300)]
        while len(cases) < 2000:
            cases.append(light(rng))
        rng.shuffle(cases)
    elif seed % 5 == 0:
        # Edge shapes: period 1, equal periods, periods dividing each other, one line.
        cases = []
        for _ in range(rng.randint(1, 5)):
            periods = [rng.choice([1, 2, 3, 4, 6, 12]) for _ in range(rng.randint(1, 4))]
            cases.append((periods, residues(rng, periods, rng.choice(["meet", "random", "never"]))))
    else:
        cases = []
        for _ in range(rng.randint(1, 5)):
            periods = [rng.randint(1, 12) for _ in range(rng.randint(1, 4))]
            cases.append((periods, residues(rng, periods, rng.choice(["meet", "random", "never"]))))
    lines = [str(len(cases))]
    for periods, rs in cases:
        lines.append(str(len(periods)))
        lines += [f"{r} {m}" for r, m in zip(rs, periods)]
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
