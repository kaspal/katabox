import re


def title_case(text: str) -> str:
    # Obviously correct: rewrite each maximal run of non-spaces with a regex.
    return re.sub(r"[^ ]+", lambda m: m.group(0)[0].upper() + m.group(0)[1:].lower(), text)
