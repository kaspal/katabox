# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def rotate_playlist(playlist: ListNode | None, k: int) -> ListNode | None:
    # Correct but O(n·(k mod n)): one walk to the end per press that matters.
    if playlist is None or playlist.next is None:
        return playlist
    n, node = 0, playlist
    while node is not None:
        n += 1
        node = node.next
    for _ in range(k % n):
        before = playlist
        while before.next.next is not None:
            before = before.next
        last = before.next
        before.next = None
        last.next = playlist
        playlist = last
    return playlist
