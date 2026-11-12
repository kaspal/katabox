class Notebook:
    def __init__(self):
        self.values: dict[int, int] = {}
        self.undo_stack: list[tuple[int, int | None, int]] = []
        self.redo_stack: list[tuple[int, int | None, int]] = []

    def set(self, key: int, value: int | None) -> None:
        if value is None:
            self.values.pop(key, None)
        else:
            self.values[key] = value

    def write(self, key: int, value: int) -> None:
        self.undo_stack.append((key, self.values.get(key), value))
        self.values[key] = value
        self.redo_stack.clear()

    def read(self, key: int) -> int:
        return self.values.get(key, -1)

    def undo(self) -> None:
        if self.undo_stack:
            key, old, new = self.undo_stack.pop()
            self.set(key, old)
            self.redo_stack.append((key, old, new))

    def redo(self) -> None:
        if self.redo_stack:
            key, old, new = self.redo_stack.pop()
            self.set(key, new)
            self.undo_stack.append((key, old, new))


def notebook(ops: list[str], args: list[list[int]]) -> list[int]:
    book = Notebook()
    reports = []
    for op, a in zip(ops, args):
        if op == "write":
            book.write(a[0], a[1])
        elif op == "read":
            reports.append(book.read(a[0]))
        elif op == "undo":
            book.undo()
        else:
            book.redo()
    return reports
