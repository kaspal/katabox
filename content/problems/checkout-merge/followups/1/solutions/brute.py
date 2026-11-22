# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def merge_distinct(lines: list[ListNode | None]) -> ListNode | None:
    # Obviously correct: every ticket number once, in order.
    tickets = set()
    for node in lines:
        while node is not None:
            tickets.add(node.val)
            node = node.next
    head = None
    for t in sorted(tickets, reverse=True):
        head = ListNode(t, head)
    return head
