def without_banned(items: list[int], banned: list[int]) -> list[int]:
    bad = set(banned)
    for x in items:
        if x in bad:
            items.remove(x)
    return items
