from functools import cmp_to_key


def biggest_number(cards: list[int]) -> str:
    texts = sorted(map(str, cards), key=cmp_to_key(lambda a, b: (b + a > a + b) - (b + a < a + b)))
    joined = "".join(texts)
    return "0" if joined[0] == "0" else joined
