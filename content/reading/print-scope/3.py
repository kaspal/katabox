count = 10


def reset():
    count = 0


def bump():
    global count
    count += 1


reset()
bump()
print(count)
