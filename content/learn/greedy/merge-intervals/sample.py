def merge(intervals):
    merged = []
    for start, end in sorted(intervals):
        if merged and start <= merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], end)  # @step extend
        else:
            merged.append([start, end])  # @step open
    return [x for pair in merged for x in pair]
