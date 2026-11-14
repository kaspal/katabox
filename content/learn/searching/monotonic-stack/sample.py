def next_greater(a):
    result = [-1] * len(a)
    stack = []
    for i in range(len(a)):
        while stack and a[stack[-1]] < a[i]:  # @step compare
            result[stack.pop()] = a[i]  # @step pop
        stack.append(i)  # @step push
    return result
