fns = []
for i in range(3):
    fns.append(lambda: i)

print(" ".join(str(f()) for f in fns))
