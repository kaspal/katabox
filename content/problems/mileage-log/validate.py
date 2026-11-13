def validate(inp):
    miles, trips = inp["miles"], inp["trips"]
    if not isinstance(miles, list) or not all(type(v) is int for v in miles):
        return "miles is a list of integers"
    if not isinstance(trips, list) or not all(isinstance(t, list) and len(t) == 2 and all(type(v) is int for v in t) for t in trips):
        return "trips is a list of [a, b] pairs"
    if not 1 <= len(miles) <= 10**5:
        return "1 ≤ miles.length ≤ 10⁵"
    if not all(0 <= v <= 1000 for v in miles):
        return "0 ≤ miles[i] ≤ 1000"
    if not 1 <= len(trips) <= 10**5:
        return "1 ≤ trips.length ≤ 10⁵"
    if not all(0 <= a <= b < len(miles) for a, b in trips):
        return "0 ≤ a ≤ b < miles.length for each trip [a, b]"
    return None
