def spell(n, acc):
    if n == 0:
        return acc
    return spell(n // 10, acc + str(n % 10))


print(spell(305, ""))
print(spell(40, ""))
