"""Random encoded camp clues as stdin text (a program problem, ADR 0028)."""
import random

WORDS = [
    "meet", "at", "the", "old", "oak", "bring", "a", "lamp", "rope", "north", "gate", "river",
    "camp", "clue", "next", "under", "stone", "bridge", "seven", "map", "look", "behind", "red",
    "door", "fox", "jump", "quiz", "wax", "key", "box", "hidden", "yard", "zebra", "vivid", "pine",
    "torch", "whistle", "cabin", "lake", "hill", "west", "east", "south", "count", "steps", "find",
    "flag", "dock", "canoe", "owl",
]
PUNCT = ".,;:!?-'\"()%#&*/"
PRINTABLE = "".join(chr(c) for c in range(32, 127))


def encode(text, s):
    out = []
    for ch in text:
        if "a" <= ch <= "z":
            out.append(chr((ord(ch) - 97 + s) % 26 + 97))
        elif "A" <= ch <= "Z":
            out.append(chr((ord(ch) - 65 + s) % 26 + 65))
        else:
            out.append(ch)
    return "".join(out)


def clue_line(rng, max_words):
    parts = []
    for i in range(rng.randint(1, max_words)):
        if rng.random() < 0.1:
            word = str(rng.randint(0, 999))
        else:
            word = rng.choice(WORDS)
            if i == 0 or rng.random() < 0.1:
                word = word.capitalize()
            elif rng.random() < 0.03:
                word = word.upper()
        if rng.random() < 0.15:
            word += rng.choice(PUNCT)
        parts.append(word)
    line = " ".join(parts)
    if rng.random() < 0.05:
        line = "  " + line  # leading spaces must survive
    return line


def noise_line(rng, longest):
    return "".join(rng.choice(PRINTABLE) for _ in range(rng.randint(0, longest)))


def generate(seed, size):
    rng = random.Random(seed)
    if size == "max":
        # About 140 000 characters of clues (at most one line more), some empty lines.
        s = rng.randint(1, 25)
        lines = []
        total = 0
        while total < 140_000:
            line = "" if rng.random() < 0.04 else clue_line(rng, 14)
            lines.append(line)
            total += len(line) + 1
    elif seed % 5 == 0:
        # Edge shapes: s at 0, 1, 13, 25; the alphabet's ends; empty and letter-free lines.
        s = rng.choice([0, 1, 13, 25])
        options = ["", "zZ aA yY bB", "...!? 2026", "  " + noise_line(rng, 12), "Zz"]
        lines = [rng.choice(options) for _ in range(rng.randint(1, 4))]
    else:
        s = rng.randint(0, 25)
        lines = [
            noise_line(rng, 15) if rng.random() < 0.3 else clue_line(rng, 5)
            for _ in range(rng.randint(1, 4))
        ]
    text = "\n".join(encode(line, s) for line in lines)
    return f"{s}\n{text}\n"


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    sys.stdout.write(generate(int(args[0]) if args else 1, args[1] if len(args) > 1 else "small"))
