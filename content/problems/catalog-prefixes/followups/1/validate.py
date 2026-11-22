"""catalog-prefixes--f1 constraints."""


def validate(inp):
    ops, words = inp["ops"], inp["words"]
    if not isinstance(ops, list) or not isinstance(words, list) or len(ops) != len(words):
        return "ops.length = words.length"
    if not 1 <= len(ops) <= 3 * 10**4:
        return "1 ≤ ops.length ≤ 3·10⁴"
    live: dict[str, int] = {}
    for op, w in zip(ops, words):
        if not isinstance(w, str) or not 1 <= len(w) <= 20 or not all("a" <= c <= "z" for c in w):
            return "each word has 1 to 20 lowercase English letters"
        if op == "add":
            live[w] = live.get(w, 0) + 1
        elif op == "remove":
            if live.get(w, 0) == 0:
                return "remove only takes a title that is in the catalogue"
            live[w] -= 1
        elif op != "count":
            return "each op is add, remove or count"
    return None
