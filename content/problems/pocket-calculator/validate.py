LIMIT = 2**53


class Bad(Exception):
    pass


def validate(inp):
    s = inp["expression"]
    if not isinstance(s, str) or not 1 <= len(s) <= 10**4:
        return "1 ≤ expression.length ≤ 10⁴"
    if any(c not in "0123456789+-*() " for c in s):
        return "only digits, + - *, parentheses and spaces"
    pos = 0
    depth = 0

    def peek():
        nonlocal pos
        while pos < len(s) and s[pos] == " ":
            pos += 1
        return s[pos] if pos < len(s) else ""

    def check(v):
        if abs(v) >= LIMIT:
            raise Bad("every intermediate value has absolute value < 2⁵³")
        return v

    def expr():
        nonlocal pos
        v = term()
        while peek() in ("+", "-") and peek() != "":
            op = s[pos]
            pos += 1
            w = term()
            v = check(v + w if op == "+" else v - w)
        return v

    def term():
        nonlocal pos
        v = factor()
        while peek() == "*":
            pos += 1
            v = check(v * factor())
        return v

    def factor():
        nonlocal pos, depth
        c = peek()
        if c == "-":
            pos += 1
            depth += 1
            if depth > 100:
                raise Bad("parentheses and unary minus signs are nested at most 100 deep")
            v = check(-factor())
            depth -= 1
            return v
        if c == "(":
            pos += 1
            depth += 1
            if depth > 100:
                raise Bad("parentheses and unary minus signs are nested at most 100 deep")
            v = expr()
            if peek() != ")":
                raise Bad("unbalanced parentheses")
            pos += 1
            depth -= 1
            return v
        start = pos
        while pos < len(s) and s[pos].isdigit():
            pos += 1
        digits = s[start:pos]
        if not digits:
            raise Bad("a number, '-' or '(' was expected")
        if len(digits) > 1 and digits[0] == "0":
            raise Bad("numbers have no leading zeros")
        return check(int(digits))

    try:
        expr()
        if peek() != "":
            return "unexpected text after the expression"
    except Bad as e:
        return str(e)
    except RecursionError:
        return "too deeply nested"
    return None
