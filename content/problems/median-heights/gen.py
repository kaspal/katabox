"""Choir sections' heights as stdin text (a program problem, ADR 0028)."""
import random


def render(sections):
    lines = [str(len(sections))]
    for heights in sections:
        lines.append(str(len(heights)))
        lines.append(" ".join(map(str, heights)))
    return "\n".join(lines) + "\n"


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        if seed % 2 == 1:
            # 1000 sections of 100 singers: the most sections and the most singers.
            return render([[rng.randint(140, 210) for _ in range(100)] for _ in range(1000)])
        # One section of 5·10⁴, heights up to 10⁹, many repeated.
        pool = [rng.randint(1, 1_000_000_000) for _ in range(500)]
        return render([[rng.choice(pool) for _ in range(50_000)]])
    if seed % 5 == 0:
        # Edge shapes: one singer, two singers, all the same, the extreme heights.
        sections = []
        for _ in range(rng.randint(1, 4)):
            kind = rng.randrange(3)
            if kind == 0:
                sections.append([rng.choice([1, 1_000_000_000])])
            elif kind == 1:
                sections.append([rng.choice([1, 1_000_000_000]), rng.choice([1, 1_000_000_000])])
            else:
                sections.append([rng.randint(1, 3)] * rng.randint(2, 6))
        return render(sections)
    return render([[rng.randint(150, 200) for _ in range(rng.randint(1, 8))] for _ in range(rng.randint(1, 4))])


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
