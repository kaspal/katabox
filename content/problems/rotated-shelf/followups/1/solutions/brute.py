def find_labels(shelf: list[int], labels: list[int]) -> list[int]:
    # Obviously correct: look each label up by scanning.
    return [shelf.index(x) if x in shelf else -1 for x in labels]
