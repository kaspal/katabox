class Node:
    def __init__(self, value):
        self.value = value
        self.prev = None
        self.next = None


class Deque:
    def __init__(self):
        self.head = None
        self.tail = None

    def push_front(self, value):
        node = Node(value)
        node.next = self.head  # @step link-front
        if self.head is not None:
            self.head.prev = node
        else:
            self.tail = node
        self.head = node

    def push_back(self, value):
        node = Node(value)
        node.prev = self.tail  # @step link-back
        if self.tail is not None:
            self.tail.next = node
        else:
            self.head = node
        self.tail = node

    def pop_front(self):
        if self.head is None:
            return -1  # @step empty-front
        node = self.head
        self.head = node.next  # @step unlink-front
        if self.head is not None:
            self.head.prev = None
        else:
            self.tail = None
        return node.value

    def pop_back(self):
        if self.tail is None:
            return -1  # @step empty-back
        node = self.tail
        self.tail = node.prev  # @step unlink-back
        if self.tail is not None:
            self.tail.next = None
        else:
            self.head = None
        return node.value


def run(ops):
    d, out = Deque(), []
    for op in ops:
        if op[0] == "front":
            d.push_front(op[1])
        elif op[0] == "back":
            d.push_back(op[1])
        elif op[0] == "pop-front":
            out.append(d.pop_front())
        else:
            out.append(d.pop_back())
    out.append(0)
    node = d.head
    while node is not None:
        out.append(node.value)
        node = node.next
    out.append(0)
    node = d.tail
    while node is not None:
        out.append(node.value)
        node = node.prev
    return out
