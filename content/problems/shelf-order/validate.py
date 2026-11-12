import re

LABEL = re.compile(r"[a-z0-9]{1,20}")


def validate(inp):
    labels = inp["labels"]
    if not isinstance(labels, list) or not 1 <= len(labels) <= 10**4:
        return "1 ≤ labels.length ≤ 10⁴"
    if not all(isinstance(s, str) and LABEL.fullmatch(s) for s in labels):
        return "labels have 1 to 20 lowercase letters and digits"
    return None
