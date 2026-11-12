import re

CHUNK = re.compile(r"[0-9]+|[a-z]+")


def chunk_key(label: str) -> list[tuple]:
    key = []
    for chunk in CHUNK.findall(label):
        if chunk[0].isdigit():
            digits = chunk.lstrip("0")
            key.append((0, len(digits), digits))
        else:
            key.append((1, chunk))
    return key


def natural_order(labels: list[str]) -> list[str]:
    return sorted(labels, key=chunk_key)  # sorted() is stable
