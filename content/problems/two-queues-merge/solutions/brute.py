# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def merge_queues(first: ListNode | None, second: ListNode | None) -> ListNode | None:
    # Obviously correct: collect every ticket, sort, rebuild.
    values = []
    for q in (first, second):
        while q is not None:
            values.append(q.val)
            q = q.next
    head = None
    for v in sorted(values, reverse=True):
        head = ListNode(v, head)
    return head
