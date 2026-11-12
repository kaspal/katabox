class Parser:
    def __init__(self, text: str):
        self.text = text
        self.pos = 0

    def peek(self) -> str:
        while self.pos < len(self.text) and self.text[self.pos] == " ":
            self.pos += 1
        return self.text[self.pos] if self.pos < len(self.text) else ""

    def expr(self) -> int:
        value = self.term()
        while self.peek() in ("+", "-") and self.peek() != "":
            op = self.text[self.pos]
            self.pos += 1
            right = self.term()
            value = value + right if op == "+" else value - right
        return value

    def term(self) -> int:
        value = self.factor()
        while self.peek() == "*":
            self.pos += 1
            value *= self.factor()
        return value

    def factor(self) -> int:
        c = self.peek()
        if c == "-":
            self.pos += 1
            return -self.factor()
        if c == "(":
            self.pos += 1
            value = self.expr()
            self.peek()
            self.pos += 1  # ")"
            return value
        start = self.pos
        while self.pos < len(self.text) and self.text[self.pos].isdigit():
            self.pos += 1
        return int(self.text[start : self.pos])


def evaluate(expression: str) -> int:
    return Parser(expression).expr()
