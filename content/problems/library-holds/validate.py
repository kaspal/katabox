ARITY = {"hold": 2, "cancel": 2, "return": 1}


def validate(inp):
    ops, args = inp["ops"], inp["args"]
    if not isinstance(ops, list) or not isinstance(args, list):
        return "ops and args are arrays"
    if not 1 <= len(ops) <= 10**5 or len(args) != len(ops):
        return "1 ≤ ops.length ≤ 10⁵ and args.length = ops.length"
    waiting = set()  # (book, member) pairs currently in a line
    lines = {}
    for op, a in zip(ops, args):
        if op not in ARITY:
            return "ops[i] is \"hold\", \"cancel\" or \"return\""
        if not isinstance(a, list) or len(a) != ARITY[op] or not all(type(x) is int for x in a):
            return "hold and cancel take [book, member]; return takes [book]"
        if not all(1 <= x <= 10**9 for x in a):
            return "1 ≤ book, member ≤ 10⁹"
        book = a[0]
        if op == "hold":
            if (book, a[1]) in waiting:
                return "a member never holds a book whose line they are already in"
            waiting.add((book, a[1]))
            lines.setdefault(book, []).append(a[1])
        elif op == "cancel":
            waiting.discard((book, a[1]))
        else:
            q = lines.get(book, [])
            while q:
                m = q.pop(0)
                if (book, m) in waiting:
                    waiting.discard((book, m))
                    break
    return None
