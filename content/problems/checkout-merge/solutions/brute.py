# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def merge_lines(lines: list[ListNode | None]) -> ListNode | None:
    # Obviously correct: collect every ticket and sort.
    tickets = []
    for node in lines:
        while node is not None:
            tickets.append(node.val)
            node = node.next
    head = None
    for v in sorted(tickets, reverse=True):
        head = ListNode(v, head)
    return head
