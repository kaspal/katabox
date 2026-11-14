"""Random field notes as stdin text (a program problem, ADR 0028)."""
import random
from itertools import accumulate

SYLLABLES = ["mo", "ra", "ti", "ne", "lu", "sa", "ko", "vi", "pe", "da", "gu", "fi", "e", "o"]
SPACED = [" "] * 6 + [", ", ". ", "; ", ": ", "! ", "? ", "...", ",", "!?", " . "]
TIGHT = [",", ".", ";", ":", "!", "?", "...", "?!", ",,"]


def vocabulary(rng, size):
    # Distinct words in generation order (never iterate a set: its order is not fixed).
    seen = set()
    words = []
    while len(words) < size:
        word = "".join(rng.choice(SYLLABLES) for _ in range(rng.randint(1, 4)))
        if word not in seen:
            seen.add(word)
            words.append(word)
    return words


def notes(rng, vocab, target, max_words, empty_share, separators):
    # Zipf-like word frequencies, so counts differ and ties still happen.
    cum = list(accumulate(1 / (i + 1) for i in range(len(vocab))))
    lines = []
    size = 0
    while size < target or not lines:
        if lines and rng.random() < empty_share:
            line = ""
        else:
            parts = []
            for _ in range(rng.randint(1, max_words)):
                parts.append(rng.choices(vocab, cum_weights=cum)[0])
                parts.append(rng.choice(separators))
            line = "".join(parts).strip()
        lines.append(line)
        size += len(line) + 1
    return lines


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # About 190 000 characters of text (at most one line more), K up to 100.
        if seed % 4 == 0:
            vocab, k = vocabulary(rng, 40), 100  # fewer distinct words than K
        else:
            vocab, k = vocabulary(rng, 2000), 100 if seed % 3 else rng.randint(1, 100)
        lines = notes(rng, vocab, 190_000, 12, 0.03, SPACED)
    elif seed % 5 == 0:
        # Edge shapes: punctuation without spaces, K far above the distinct words, K = 1.
        vocab = vocabulary(rng, rng.randint(1, 3))
        k = rng.choice([1, 100, rng.randint(1, 4)])
        lines = notes(rng, vocab, rng.randint(1, 40), 5, 0.3, TIGHT + SPACED)
    else:
        vocab = vocabulary(rng, rng.randint(1, 6))
        k = rng.randint(1, 5)
        lines = notes(rng, vocab, rng.randint(1, 60), 5, 0.15, SPACED)
    return "\n".join([str(k), *lines]) + "\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
