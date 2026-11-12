// Provided by KataBox: the judge builds the input lists with this class.
export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

export function rotatePlaylist(playlist: ListNode | null, k: number): ListNode | null {
  if (playlist === null) return null;
  let n = 1;
  let tail = playlist;
  while (tail.next !== null) {
    tail = tail.next;
    n++;
  }
  const r = k % n;
  if (r === 0) return playlist;
  let newTail = playlist;
  for (let i = 0; i < n - r - 1; i++) newTail = newTail.next!;
  const head = newTail.next;
  newTail.next = null;
  tail.next = playlist;
  return head;
}
