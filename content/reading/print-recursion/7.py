def ping(n):
    if n == 0:
        return "ping"
    return pong(n - 1)


def pong(n):
    if n == 0:
        return "pong"
    return ping(n - 1)


print(ping(3))
print(ping(4))
