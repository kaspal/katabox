# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def zigzag_boarding(queue: ListNode | None) -> ListNode | None:
    # Correct but O(n²): after each placed passenger, walk to the end for the last one.
    node = queue
    while node is not None and node.next is not None and node.next.next is not None:
        before = node
        while before.next.next is not None:
            before = before.next
        last = before.next
        before.next = None
        last.next = node.next
        node.next = last
        node = last.next
    return queue
