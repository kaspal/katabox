# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def split_train(train: ListNode | None, k: int) -> list[ListNode | None]:
    # Obviously correct: copy the values, hand them out by size, rebuild each part.
    values = []
    while train is not None:
        values.append(train.val)
        train = train.next
    sizes = [len(values) // k + (1 if i < len(values) % k else 0) for i in range(k)]
    parts, pos = [], 0
    for s in sizes:
        head = None
        for v in reversed(values[pos : pos + s]):
            head = ListNode(v, head)
        parts.append(head)
        pos += s
    return parts
