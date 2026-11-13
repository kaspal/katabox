# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def reverse_batches(line: ListNode | None, k: int) -> ListNode | None:
    # Obviously correct: copy the values, reverse full slices, rebuild.
    values = []
    while line is not None:
        values.append(line.val)
        line = line.next
    out = []
    for start in range(0, len(values), k):
        chunk = values[start : start + k]
        out.extend(reversed(chunk) if len(chunk) == k else chunk)
    head = None
    for v in reversed(out):
        head = ListNode(v, head)
    return head
