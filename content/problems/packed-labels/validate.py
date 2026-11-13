import re

LABEL = re.compile(r"(?:[1-9][0-9]*[a-z])+")
RUN = re.compile(r"([0-9]+)([a-z])")


def is_int(s):
    return s.isascii() and s.isdigit()


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not is_int(lines[0].strip()):
        return "the first line is L"
    n_labels = int(lines[0])
    if not 1 <= n_labels <= 1000:
        return "1 ≤ L ≤ 1000"
    if len(lines) < n_labels + 2:
        return "L label lines, then Q"
    lengths = []
    total_runs = 0
    for idx in range(1, n_labels + 1):
        label = lines[idx].strip()
        if not LABEL.fullmatch(label):
            return f"label {idx}: runs of a count without leading zeros and a letter a-z"
        length = 0
        for count, _ in RUN.findall(label):
            if len(count) > 10 or int(count) > 10**9:
                return "1 ≤ count ≤ 10⁹"
            length += int(count)
            total_runs += 1
        lengths.append(length)
    if total_runs > 60_000:
        return "at most 60 000 runs in all the labels together"
    q_line = lines[n_labels + 1].strip()
    if not is_int(q_line):
        return "the line after the labels is Q"
    q = int(q_line)
    if not 1 <= q <= 20_000:
        return "1 ≤ Q ≤ 20 000"
    if len(lines) != n_labels + 2 + q:
        return "each of the Q checks takes one line"
    for line in lines[n_labels + 2 :]:
        parts = line.split()
        if len(parts) != 2 or not all(is_int(p) for p in parts):
            return "each check line holds two numbers i k"
        i, k = int(parts[0]), int(parts[1])
        if not 1 <= i <= n_labels:
            return "1 ≤ i ≤ L"
        if not 1 <= k <= lengths[i - 1]:
            return "1 ≤ k ≤ the unpacked length of label i"
    return None
