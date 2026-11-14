class DynArray:
    def __init__(self):
        self.data = [0] * 2
        self.size = 0

    def insert(self, i, value):
        if self.size == len(self.data):
            self.data = self.data + [0] * len(self.data)  # @step grow
        for k in range(self.size, i, -1):
            self.data[k] = self.data[k - 1]  # @step shift-right
        self.data[i] = value  # @step write
        self.size += 1

    def remove(self, i):
        for k in range(i, self.size - 1):
            self.data[k] = self.data[k + 1]  # @step shift-left
        self.size -= 1  # @step shrink


def run(ops):
    arr = DynArray()
    for op in ops:
        if op[0] == "append":
            arr.insert(arr.size, op[1])
        elif op[0] == "insert":
            arr.insert(min(op[1], arr.size), op[2])
        elif arr.size > 0:
            arr.remove(min(op[1], arr.size - 1))
    return arr.data[:arr.size]
