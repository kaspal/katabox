a = [1, 2, 3]
b = a[:]
c = list(a)
b[0] = 10
c.append(99)
print(" ".join(map(str, a)))
print(" ".join(map(str, b)))
print(len(c))
