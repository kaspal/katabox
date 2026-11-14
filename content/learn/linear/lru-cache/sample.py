from collections import OrderedDict


class LRUCache:
    def __init__(self, capacity):
        self.capacity = capacity
        self.items = OrderedDict()

    def get(self, key):
        if key not in self.items:
            return -1  # @step miss
        self.items.move_to_end(key)  # @step hit
        return self.items[key]

    def put(self, key, value):
        if key in self.items:
            self.items.move_to_end(key)  # @step refresh
        self.items[key] = value  # @step put
        if len(self.items) > self.capacity:
            self.items.popitem(last=False)  # @step evict


def run(capacity, ops):
    cache, out = LRUCache(capacity), []
    for op in ops:
        if op[0] == "get":
            out.append(cache.get(op[1]))
        else:
            cache.put(op[1], op[2])
    return out
