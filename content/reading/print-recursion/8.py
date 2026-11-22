def rev(s):
    if len(s) <= 1:
        return s
    return rev(s[1:]) + s[0]


def total(xs):
    if not xs:
        return 0
    return xs[0] + total(xs[1:])


print(rev("abc"))
print(total([4, 5, 6]))
