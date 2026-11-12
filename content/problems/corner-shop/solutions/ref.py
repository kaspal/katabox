class Shop:
    def __init__(self):
        self.shelf: dict[int, int] = {}

    def stock(self, item: int, qty: int) -> None:
        self.shelf[item] = self.shelf.get(item, 0) + qty

    def sell(self, item: int, qty: int) -> int:
        sold = min(qty, self.shelf.get(item, 0))
        self.shelf[item] = self.shelf.get(item, 0) - sold
        return sold

    def count(self, item: int) -> int:
        return self.shelf.get(item, 0)


def shop(ops: list[str], args: list[list[int]]) -> list[int]:
    keeper = Shop()
    reports = []
    for op, a in zip(ops, args):
        if op == "stock":
            keeper.stock(a[0], a[1])
        elif op == "sell":
            reports.append(keeper.sell(a[0], a[1]))
        else:
            reports.append(keeper.count(a[0]))
    return reports
