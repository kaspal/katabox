def peak_index(heights: list[int]) -> int:
    # Obviously correct: the position of the highest marker.
    return heights.index(max(heights))
