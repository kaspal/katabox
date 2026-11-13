"""Random volunteers; one big case needs one volunteer per skill."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        m = 14
        if seed % 2:
            # Everyone has exactly one skill: the answer is 14.
            people = [1 << (i % m) for i in range(60)]
            rng.shuffle(people)
            return {"skills": m, "people": people}
        people = []
        for _ in range(60):
            mask = 0
            for _ in range(rng.randint(1, 3)):
                mask |= 1 << rng.randrange(m)
            people.append(mask)
        return {"skills": m, "people": people}
    m = rng.randint(1, 5)
    return {"skills": m, "people": [rng.randrange(1 << m) for _ in range(rng.randint(1, 6))]}


if __name__ == "__main__":
    import json
    import sys

    args = sys.argv[1:]
    if "--n" in args:
        # Profile size (ADR 0024): python gen.py --n N [--seed S]
        seed = int(args[args.index("--seed") + 1]) if "--seed" in args else 1
        print(json.dumps(generate(seed, int(args[args.index("--n") + 1]))))
    else:
        print(json.dumps(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small")))
