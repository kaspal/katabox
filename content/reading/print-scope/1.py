def make_counter():
    count = 0

    def step():
        nonlocal count
        count += 1
        return count

    return step


a = make_counter()
b = make_counter()
print(a())
print(a())
print(b())
