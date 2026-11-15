import re

NAME = re.compile(r"[a-z]{1,12}")
LAP = re.compile(r"([0-9]{1,2}):([0-5][0-9])")


def validate(text):
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or not re.fullmatch(r"[0-9]+", lines[0]):
        return "the first line is T"
    t = int(lines[0])
    if not 1 <= t <= 100:
        return "1 ≤ T ≤ 100"
    if len(lines) != 1 + t:
        return "one line per runner"
    for line in lines[1:]:
        parts = line.split(" ")
        if len(parts) < 3 or not NAME.fullmatch(parts[0]):
            return "each line starts with a name of 1-12 lowercase letters"
        if not re.fullmatch(r"[0-9]+", parts[1]):
            return "then k"
        k = int(parts[1])
        if not 1 <= k <= 50:
            return "1 ≤ k ≤ 50"
        if len(parts) != 2 + k:
            return "then exactly k laps"
        for p in parts[2:]:
            m = LAP.fullmatch(p)
            if not m:
                return "a lap is m:ss"
            minutes, seconds = int(m.group(1)), int(m.group(2))
            if p != f"{minutes}:{seconds:02d}":
                return "minutes have no leading zero"
            if not 1 <= minutes * 60 + seconds <= 3599:
                return "0:01 ≤ lap ≤ 59:59"
    return None
