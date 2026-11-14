M = 7


def build(keys):
    table = [[] for _ in range(M)]
    for key in keys:
        b = key % M  # @step hash
        if key in table[b]:  # @step scan
            continue
        table[b].append(key)  # @step insert
    return table


def run(keys):
    out = []
    for bucket in build(keys):
        out.append(len(bucket))
        out.extend(bucket)
    return out
