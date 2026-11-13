# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def split_train(train: ListNode | None, k: int) -> list[ListNode | None]:
    n, node = 0, train
    while node is not None:
        n += 1
        node = node.next
    base, extra = divmod(n, k)
    parts, node = [], train
    for i in range(k):
        size = base + (1 if i < extra else 0)
        if size == 0:
            parts.append(None)
            continue
        parts.append(node)
        for _ in range(size - 1):
            node = node.next
        node.next, node = None, node.next
    return parts
