def wrap(n):
    if n == 0:
        print("base")
        return
    print("in", n)
    wrap(n - 1)
    print("out", n)


wrap(2)
