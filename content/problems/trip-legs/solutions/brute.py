"""Every booked leg with both cities, in plain Python."""


def solve(tables):
    city = dict(tables["airports"])
    flights = {fid: (no, src, dst, dep) for fid, no, src, dst, dep in tables["flights"]}
    rows = []
    for bid, passenger, fid in tables["bookings"]:
        no, src, dst, dep = flights[fid]
        rows.append((passenger, dep, bid, no, city[src], city[dst]))
    rows.sort()
    return {"columns": ["passenger", "flight_no", "from_city", "to_city"],
            "rows": [[p, no, a, b] for p, _, _, no, a, b in rows]}
