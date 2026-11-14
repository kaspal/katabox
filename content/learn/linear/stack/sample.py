def run(ops):
    stack, out = [], []
    for op in ops:
        if op[0] == "push":
            stack.append(op[1])  # @step push
        elif op[0] == "pop":
            out.append(stack.pop() if stack else -1)  # @step pop
        else:
            out.append(stack[-1] if stack else -1)  # @step peek
    return out
