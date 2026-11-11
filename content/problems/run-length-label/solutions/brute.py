from itertools import groupby


def compress(text: str) -> str:
    # Obviously correct: the standard library groups the runs.
    out = ""
    for letter, run in groupby(text):
        k = len(list(run))
        out += letter + (str(k) if k > 1 else "")
    return out
