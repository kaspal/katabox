def without_banned(items: list[int], banned: list[int]) -> list[int]:
    bad = set(banned)
    return [x for x in items if x not in bad]
