out = []


def tree(n):
    if n == 0:
        return
    tree(n - 1)
    out.append(str(n))
    tree(n - 1)


tree(3)
print(" ".join(out))
