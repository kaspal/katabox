# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def rotate_playlist(playlist: ListNode | None, k: int) -> ListNode | None:
    if playlist is None:
        return None
    n, tail = 1, playlist
    while tail.next is not None:
        tail = tail.next
        n += 1
    r = k % n
    if r == 0:
        return playlist
    new_tail = playlist
    for _ in range(n - r - 1):
        new_tail = new_tail.next
    head = new_tail.next
    new_tail.next = None
    tail.next = playlist
    return head
