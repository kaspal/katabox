def make_acc(start):
    total = start

    def add(x):
        nonlocal total
        total += x
        return total

    return add


acc = make_acc(10)
acc(5)
print(acc(1))
other = make_acc(0)
print(other(2))
print(acc(0))
