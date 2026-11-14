def fib_memo(n, memo):
    if n in memo:
        return memo[n]  # @step hit
    if n < 2:
        result = n
    else:
        result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
    memo[n] = result  # @step store
    return result


def fib_table(n):
    table = [0] * (n + 1)
    if n > 0:
        table[1] = 1
    for i in range(2, n + 1):
        table[i] = table[i - 1] + table[i - 2]  # @step fill
    return table


def run(n):
    return [fib_memo(n, {})] + fib_table(n)
