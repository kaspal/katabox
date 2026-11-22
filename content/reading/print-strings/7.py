s = ""
for i in range(1, 6):
    if i % 2 == 1:
        s = s + str(i)
    else:
        s = str(i) + s
print(s)
