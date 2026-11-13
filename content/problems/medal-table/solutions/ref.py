def medal_table(countries: list[str], medals: list[list[int]]) -> list[str]:
    order = sorted(range(len(countries)),
                   key=lambda i: (-medals[i][0], -medals[i][1], -medals[i][2], countries[i]))
    return [countries[i] for i in order]
