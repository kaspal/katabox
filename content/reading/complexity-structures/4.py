def f(n):
    vals = [(i * 37) % 101 for i in range(n)]
    answer = [-1] * n
    stack = []
    for i in range(n):
        while stack and vals[stack[-1]] < vals[i]:
            answer[stack.pop()] = i
        stack.append(i)
    return answer
