import heapq

# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def fair_sample(lines: list[ListNode | None]) -> list[int]:
    # Obviously correct for small inputs: try every pair of ticket values as [lo, hi].
    tickets = []
    for node in lines:
        values = []
        while node is not None:
            values.append(node.val)
            node = node.next
        tickets.append(values)
    candidates = sorted({v for values in tickets for v in values})
    best = None
    for lo in candidates:
        for hi in candidates:
            if hi < lo:
                continue
            if all(any(lo <= v <= hi for v in values) for values in tickets):
                if best is None or (hi - lo, lo) < (best[1] - best[0], best[0]):
                    best = [lo, hi]
    return best
