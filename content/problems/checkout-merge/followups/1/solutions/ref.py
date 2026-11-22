import heapq

# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def fair_sample(lines: list[ListNode | None]) -> list[int]:
    heap = []
    largest = 0
    for i, node in enumerate(lines):
        heapq.heappush(heap, (node.val, i, node))
        largest = max(largest, node.val)
    best = None
    while True:
        low, i, node = heapq.heappop(heap)
        if best is None or largest - low < best[1] - best[0]:
            best = [low, largest]
        if node.next is None:
            return best
        node = node.next
        largest = max(largest, node.val)
        heapq.heappush(heap, (node.val, i, node))
