def first_matches(words: list[str], prefixes: list[str]) -> list[str]:
    # Obviously correct: the smallest of the words that start with the prefix.
    return [min((w for w in words if w.startswith(p)), default="") for p in prefixes]
