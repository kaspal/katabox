def lone_sock(ids: list[int]) -> int:
    # Correct but O(n²): look for each sock's partner elsewhere in the list.
    n = len(ids)
    for i in range(n):
        x = ids[i]
        paired = False
        for j in range(n):
            if j != i and ids[j] == x:
                paired = True
                break
        if not paired:
            return x
    return -1
