def rod_cutting(prices):
    n = len(prices)
    best = [0] * (n + 1)
    first = [0] * (n + 1)
    for length in range(1, n + 1):
        for cut in range(1, length + 1):
            if prices[cut - 1] + best[length - cut] > best[length]:
                best[length] = prices[cut - 1] + best[length - cut]  # @step improve
                first[length] = cut
    pieces, length = [], n
    while length > 0:
        pieces.append(first[length])  # @step trace
        length -= first[length]
    return [best[n]] + pieces
