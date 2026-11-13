# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def zigzag_boarding(queue: ListNode | None) -> ListNode | None:
    if queue is None or queue.next is None:
        return queue
    slow, fast = queue, queue.next
    while fast is not None and fast.next is not None:
        slow, fast = slow.next, fast.next.next
    back, slow.next = slow.next, None
    prev = None
    while back is not None:
        back.next, prev, back = prev, back, back.next
    front, back = queue, prev
    while back is not None:
        front_next, back_next = front.next, back.next
        front.next, back.next = back, front_next
        front, back = front_next, back_next
    return queue
