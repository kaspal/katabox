BASE, MOD = 26, 101


def window_hashes(text, length):
    power = 1
    for _ in range(length - 1):
        power = power * BASE % MOD
    h = 0
    for c in text[:length]:
        h = (h * BASE + ord(c) - 97) % MOD  # @step add
    out = [h]
    for i in range(length, len(text)):
        h = (h - (ord(text[i - length]) - 97) * power) % MOD  # @step remove
        h = (h * BASE + ord(text[i]) - 97) % MOD  # @step shift-in
        out.append(h)
    return out
