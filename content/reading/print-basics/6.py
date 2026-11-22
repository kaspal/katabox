found = 0
for i in range(3):
    for j in range(3):
        if j == i:
            break
        found += 1
print(found)
