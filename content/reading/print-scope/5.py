def add(item, bag=[]):
    bag.append(item)
    return bag


print(len(add("a")))
print(len(add("b")))
print(len(add("c", [])))
print(len(add("d")))
