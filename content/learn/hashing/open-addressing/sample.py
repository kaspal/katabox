M = 8
EMPTY, DELETED = -1, -2


class Table:
    def __init__(self):
        self.slots = [EMPTY] * M

    def find_slot(self, key):
        i = key % M  # @step hash
        while self.slots[i] != EMPTY and self.slots[i] != key:
            i = (i + 1) % M  # @step probe
        return i

    def insert(self, key):
        i = self.find_slot(key)
        self.slots[i] = key  # @step place

    def find(self, key):
        return 1 if self.slots[self.find_slot(key)] == key else 0  # @step found

    def delete(self, key):
        i = self.find_slot(key)
        if self.slots[i] == key:
            self.slots[i] = DELETED  # @step tombstone


def run(ops):
    table, out = Table(), []
    for op in ops:
        if op[0] == "insert":
            table.insert(op[1])
        elif op[0] == "find":
            out.append(table.find(op[1]))
        else:
            table.delete(op[1])
    return out
