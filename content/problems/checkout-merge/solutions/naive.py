# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def merge_lines(lines: list[ListNode | None]) -> ListNode | None:
    # Correct but O(N·k): look at every line's front for each customer.
    fronts = list(lines)
    dummy = tail = ListNode()
    while True:
        best = -1
        for i, node in enumerate(fronts):
            if node is not None and (best < 0 or node.val < fronts[best].val):
                best = i
        if best < 0:
            break
        node = fronts[best]
        fronts[best] = node.next
        tail.next = node
        tail = node
    tail.next = None
    return dummy.next
