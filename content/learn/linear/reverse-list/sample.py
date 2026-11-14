class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


def build(values):
    head = None
    for value in reversed(values):
        node = Node(value)
        node.next = head
        head = node
    return head


def reverse(head):
    prev, cur = None, head
    while cur is not None:
        nxt = cur.next  # @step save
        cur.next = prev  # @step flip
        prev, cur = cur, nxt  # @step advance
    return prev  # @step new-head


def run(values):
    head, out = reverse(build(values)), []
    while head is not None:
        out.append(head.value)
        head = head.next
    return out
