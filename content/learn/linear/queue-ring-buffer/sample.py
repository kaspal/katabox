class RingBuffer:
    def __init__(self, capacity):
        self.data = [0] * capacity
        self.head = 0
        self.size = 0

    def enqueue(self, value):
        if self.size == len(self.data):
            return False  # @step full
        tail = (self.head + self.size) % len(self.data)
        self.data[tail] = value  # @step enqueue
        self.size += 1
        return True

    def dequeue(self):
        if self.size == 0:
            return -1  # @step empty
        value = self.data[self.head]  # @step dequeue
        self.head = (self.head + 1) % len(self.data)
        self.size -= 1
        return value


def run(capacity, ops):
    queue, out = RingBuffer(capacity), []
    for op in ops:
        if op[0] == "enqueue":
            queue.enqueue(op[1])
        else:
            out.append(queue.dequeue())
    return out
