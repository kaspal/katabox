class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


def run(ops):
    head = None
    for op in ops:
        if op[0] == "front":
            node = Node(op[1])
            node.next = head  # @step link
            head = node  # @step new-head
        elif op[0] == "back":
            node = Node(op[1])
            if head is None:
                head = node  # @step first
            else:
                cur = head
                while cur.next is not None:
                    cur = cur.next  # @step walk
                cur.next = node  # @step append
        else:
            prev, cur = None, head
            while cur is not None and cur.value != op[1]:
                prev, cur = cur, cur.next  # @step search
            if cur is not None:
                if prev is None:
                    head = cur.next  # @step unlink-head
                else:
                    prev.next = cur.next  # @step unlink
    out = []
    while head is not None:
        out.append(head.value)
        head = head.next
    return out
