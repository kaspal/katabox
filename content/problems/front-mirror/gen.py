"""Shop-sign words as stdin text (a program problem, ADR 0028)."""
import random


def lopsided(n):
    """a…a b a…a with twice as many a's after the b: many prefixes are almost
    palindromes, so checking prefixes from the longest down takes about n²/9 steps."""
    m = n // 3
    return "a" * m + "b" + "a" * (n - m - 1)


def sized(rng, n):
    """Complexity profile input (ADR 0024): a lopsided word of n letters."""
    return lopsided(n) + "\n"


def generate(seed, size):
    rng = random.Random(seed)
    if isinstance(size, int):
        return sized(rng, size)
    if size == "max":
        if seed % 2 == 1:
            # 2·10⁵ letters, lopsided: about 4.4·10⁹ letter comparisons the slow way.
            return lopsided(200_000) + "\n"
        # 2·10⁵ random letters from a and b, starting with a long palindrome.
        half = "".join(rng.choice("ab") for _ in range(30_000))
        word = half + "c" + half[::-1]
        word += "".join(rng.choice("ab") for _ in range(200_000 - len(word)))
        return word + "\n"
    if seed % 5 == 0:
        # Edge shapes: one letter, already a palindrome, no palindromic prefix longer
        # than one letter, all the same letter.
        kind = rng.randrange(4)
        return ["q", "racecar", "abcdefg", "z" * rng.randint(2, 9)][kind] + "\n"
    return "".join(rng.choice("abc"[: rng.randint(1, 3)]) for _ in range(rng.randint(1, 10))) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
