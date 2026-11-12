from collections import deque


class Library:
    def __init__(self):
        self.line: dict[int, deque[tuple[int, int]]] = {}
        self.waiting: dict[tuple[int, int], int] = {}
        self.tickets = 0

    def hold(self, book: int, member: int) -> None:
        self.tickets += 1
        self.waiting[(book, member)] = self.tickets
        self.line.setdefault(book, deque()).append((member, self.tickets))

    def cancel(self, book: int, member: int) -> None:
        self.waiting.pop((book, member), None)

    def give_back(self, book: int) -> int:
        queue = self.line.get(book)
        while queue:
            member, ticket = queue.popleft()
            if self.waiting.get((book, member)) == ticket:
                del self.waiting[(book, member)]
                return member
        return -1


def holds(ops: list[str], args: list[list[int]]) -> list[int]:
    library = Library()
    reports = []
    for op, a in zip(ops, args):
        if op == "hold":
            library.hold(a[0], a[1])
        elif op == "cancel":
            library.cancel(a[0], a[1])
        else:
            reports.append(library.give_back(a[0]))
    return reports
