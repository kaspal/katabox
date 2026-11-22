"""tree-height constraints."""


def validate(inp):
    t = inp["root"]
    if not isinstance(t, list) or len(t) > 2 * 10**4:
        return "the tree has at most 10⁴ nodes"
    if t and t[0] is None:
        return "the root is not null"
    nodes = [x for x in t if x is not None]
    if len(nodes) > 10**4 or not all(type(x) is int and -100 <= x <= 100 for x in nodes):
        return "at most 10⁴ nodes, −100 ≤ value ≤ 100"
    return None
