def constant(n):
    steps = 1  # @step constant
    return steps


def logarithmic(n):
    steps = 0
    i = 1
    while i <= n:
        steps += 1  # @step log
        i *= 2
    return steps


def linear(n):
    steps = 0
    for i in range(n):
        steps += 1  # @step linear
    return steps


def linearithmic(n):
    steps = 0
    for i in range(n):
        j = 1
        while j <= n:
            steps += 1  # @step nlogn
            j *= 2
    return steps


def quadratic(n):
    steps = 0
    for i in range(n):
        for j in range(n):
            steps += 1  # @step quadratic
    return steps


def exponential(n):
    steps = 0
    for subset in range(2 ** n):
        steps += 1  # @step exponential
    return steps


def count_steps(n):
    return [constant(n), logarithmic(n), linear(n),
            linearithmic(n), quadratic(n), exponential(n)]
