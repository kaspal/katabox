def validate(inp):
    jobs, lathes = inp["jobs"], inp["lathes"]
    if not isinstance(jobs, list) or not all(type(x) is int for x in jobs) or type(lathes) is not int:
        return "jobs is an array of integers; lathes is an integer"
    if not 1 <= len(jobs) <= 10**5:
        return "1 ≤ jobs.length ≤ 10⁵"
    if not all(1 <= x <= 10**4 for x in jobs):
        return "1 ≤ jobs[i] ≤ 10⁴"
    if not 1 <= lathes <= 10**5:
        return "1 ≤ lathes ≤ 10⁵"
    return None
