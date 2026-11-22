"""mileage-log--f1 constraints."""


def validate(inp):
    miles, trips = inp["miles"], inp["trips"]
    if not isinstance(miles, list) or not 1 <= len(miles) <= 10**5:
        return "1 ≤ miles.length ≤ 10⁵"
    if not all(type(m) is int and 0 <= m <= 1000 for m in miles):
        return "0 ≤ miles[i] ≤ 1000"
    if not isinstance(trips, list) or not 1 <= len(trips) <= 10**5:
        return "1 ≤ trips.length ≤ 10⁵"
    n = len(miles)
    for t in trips:
        if not (isinstance(t, list) and len(t) == 3 and all(type(v) is int for v in t)):
            return "trips[j] = [a, b, x]"
        a, b, x = t
        if not (0 <= a <= b < n and 0 <= x <= 1000):
            return "0 ≤ a ≤ b < miles.length and 0 ≤ x ≤ 1000"
    return None
