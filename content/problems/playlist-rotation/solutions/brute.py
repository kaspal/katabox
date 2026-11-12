# Provided by KataBox: the judge builds the input lists with this class.
class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def rotate_playlist(playlist: ListNode | None, k: int) -> ListNode | None:
    # Obviously correct: copy the values and press the button k times, one by one.
    songs = []
    while playlist is not None:
        songs.append(playlist.val)
        playlist = playlist.next
    for _ in range(k):
        if songs:
            songs.insert(0, songs.pop())
    head = None
    for v in reversed(songs):
        head = ListNode(v, head)
    return head
