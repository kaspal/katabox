import heapq

# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def merge_lines(lines: list[ListNode | None]) -> ListNode | None:
    fronts = list(lines)
    heap = [(node.val, i) for i, node in enumerate(fronts) if node is not None]
    heapq.heapify(heap)
    dummy = tail = ListNode()
    while heap:
        _, i = heapq.heappop(heap)
        node = fronts[i]
        fronts[i] = node.next
        tail.next = node
        tail = node
        if node.next is not None:
            heapq.heappush(heap, (node.next.val, i))
    tail.next = None
    return dummy.next
