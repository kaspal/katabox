def outer():
    msg = "hi"

    def say():
        return msg

    msg = "bye"
    return say


print(outer()())
