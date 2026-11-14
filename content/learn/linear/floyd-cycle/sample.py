class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


def build(values, pos):
    nodes = [Node(value) for value in values]
    for k in range(len(nodes) - 1):
        nodes[k].next = nodes[k + 1]
    if pos >= 0:
        nodes[-1].next = nodes[pos]
    return nodes


def cycle_start(head):
    slow = fast = head
    while fast is not None and fast.next is not None:
        slow = slow.next
        fast = fast.next.next  # @step move
        if slow is fast:
            break  # @step meet
    if fast is None or fast.next is None:
        return None  # @step no-cycle
    slow = head  # @step reset
    while slow is not fast:
        slow = slow.next
        fast = fast.next  # @step walk
    return slow  # @step found


def run(values, pos):
    nodes = build(values, pos)
    start = cycle_start(nodes[0])
    return [nodes.index(start) if start is not None else -1]
