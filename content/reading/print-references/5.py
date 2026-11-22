def add_item(xs):
    xs.append("x")


def replace(xs):
    xs = ["new"]
    xs.append("y")


items = ["a"]
add_item(items)
replace(items)
print(" ".join(items))
print(len(items))
