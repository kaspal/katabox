def medal_table(countries: list[str], medals: list[list[int]]) -> list[str]:
    # Obviously correct: repeatedly take the country that beats every other remaining one.
    def beats(a, b):
        for k in range(3):
            if medals[a][k] != medals[b][k]:
                return medals[a][k] > medals[b][k]
        return countries[a] < countries[b]

    left = list(range(len(countries)))
    out = []
    while left:
        best = left[0]
        for i in left[1:]:
            if beats(i, best):
                best = i
        out.append(countries[best])
        left.remove(best)
    return out
