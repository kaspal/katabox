{{sample}}

# --- KataBox Learn sample check (ADR 0029): read the values, call the sample on the
# array `a` (the call may also set `a` to the sample's result), print it. kb-pack wraps sample.py in this; never shown in the app.
import sys as _kb_sys


def kb_ops(values, names, width):
    """Stored operations as tuples: (name, arg, ...), e.g. ("push", 3) or ("pop",)."""
    out = []
    for k in range(0, len(values), width + 1):
        name = names[values[k] - 1]
        out.append((name, *values[k + 1:k + 1 + width]))
    return out


def kb_text(values):
    """Stored words (letter codes 0-25, words separated by -1) as strings."""
    words = [""]
    for v in values:
        if v == -1:
            words.append("")
        else:
            words[-1] += chr(97 + v)
    return words


def kb_chunks(values, size):
    """Stored groups (pairs, edges) as lists of `size` values."""
    return [list(values[k:k + size]) for k in range(0, len(values), size)]


a = [int(_kb_token) for _kb_token in _kb_sys.stdin.read().split()]
{{call}}
print(" ".join(str(_kb_value) for _kb_value in a))
