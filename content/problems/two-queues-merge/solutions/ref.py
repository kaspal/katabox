# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def merge_queues(first: ListNode | None, second: ListNode | None) -> ListNode | None:
    head = tail = ListNode()
    while first is not None and second is not None:
        if first.val <= second.val:
            tail.next, first = first, first.next
        else:
            tail.next, second = second, second.next
        tail = tail.next
    tail.next = first if first is not None else second
    return head.next
