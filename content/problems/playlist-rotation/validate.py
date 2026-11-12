def validate(inp):
    playlist, k = inp["playlist"], inp["k"]
    if not isinstance(playlist, list) or not all(type(v) is int for v in playlist) or type(k) is not int:
        return "playlist is a list of integers; k is an integer"
    if len(playlist) > 10**5:
        return "0 ≤ number of songs ≤ 10⁵"
    if not all(1 <= v <= 10**6 for v in playlist):
        return "1 ≤ song id ≤ 10⁶"
    if not 0 <= k <= 10**9:
        return "0 ≤ k ≤ 10⁹"
    return None
