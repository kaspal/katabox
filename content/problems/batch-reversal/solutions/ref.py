# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def reverse_batches(line: ListNode | None, k: int) -> ListNode | None:
    dummy = ListNode(0, line)
    before = dummy
    while True:
        probe = before
        for _ in range(k):
            probe = probe.next
            if probe is None:
                return dummy.next
        first = before.next
        prev, node = None, first
        for _ in range(k):
            node.next, prev, node = prev, node, node.next
        before.next = prev
        first.next = node
        before = first
