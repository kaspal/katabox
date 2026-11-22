def make_pair():
    n = 0

    def inc():
        nonlocal n
        n += 10

    def get():
        return n

    return inc, get


inc, get = make_pair()
inc()
inc()
print(get())
