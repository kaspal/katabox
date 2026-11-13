def find_label(shelf: list[int], label: int) -> int:
    # Obviously correct: look at every folder.
    return shelf.index(label) if label in shelf else -1
