"""Job queues and lathe counts."""
import random


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # 10⁵ jobs on 5·10⁴ lathes: scanning every lathe per job is 5·10⁹ steps.
        # Jobs under 1000 minutes keep the file small; the constraint allows 10⁴.
        n = 100_000
        jobs = [rng.randint(1, 999) for _ in range(n)]
        lathes = 50_000
    else:
        jobs = [rng.randint(1, 6) for _ in range(rng.randint(1, 8))]
        lathes = rng.randint(1, 4)
    return {"jobs": jobs, "lathes": lathes}


if __name__ == "__main__":
    import json
    import sys

    print(json.dumps(generate(int(sys.argv[1]) if len(sys.argv) > 1 else 1, sys.argv[2] if len(sys.argv) > 2 else "small")))
