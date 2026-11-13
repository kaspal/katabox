# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def zigzag_boarding(queue: ListNode | None) -> ListNode | None:
    # Obviously correct: copy the values, pick them from both ends, rebuild.
    values = []
    while queue is not None:
        values.append(queue.val)
        queue = queue.next
    order, lo, hi = [], 0, len(values) - 1
    while lo <= hi:
        order.append(values[lo])
        if lo != hi:
            order.append(values[hi])
        lo, hi = lo + 1, hi - 1
    head = None
    for v in reversed(order):
        head = ListNode(v, head)
    return head
