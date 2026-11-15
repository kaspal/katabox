"""Piers within three crossings of Central, by breadth-first search in plain Python."""


def solve(tables):
    out = {}
    for _, a, b in tables["routes"]:
        out.setdefault(a, []).append(b)
    dist = {"Central": 0}
    frontier = ["Central"]
    for hops in range(1, 4):
        nxt = []
        for p in frontier:
            for q in out.get(p, []):
                if q not in dist:
                    dist[q] = hops
                    nxt.append(q)
        frontier = nxt
    rows = sorted((h, p) for p, h in dist.items() if p != "Central")
    return {"columns": ["pier", "hops"], "rows": [[p, h] for h, p in rows]}
