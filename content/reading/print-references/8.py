a = [1]
b = a
a += [2]
a = a + [3]
a.append(4)
print(" ".join(map(str, a)))
print(" ".join(map(str, b)))
