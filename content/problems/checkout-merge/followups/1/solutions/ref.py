# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


import heapq


def merge_distinct(lines: list[ListNode | None]) -> ListNode | None:
    heap = [(node.val, i, node) for i, node in enumerate(lines) if node is not None]
    heapq.heapify(heap)
    dummy = tail = ListNode()
    last = None
    while heap:
        val, i, node = heapq.heappop(heap)
        if node.next is not None:
            heapq.heappush(heap, (node.next.val, i, node.next))
        if val != last:
            tail.next = ListNode(val)
            tail = tail.next
            last = val
    return dummy.next
