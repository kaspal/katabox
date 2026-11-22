"""till-total constraints."""
import re

PRICE = re.compile(r"(0|[1-9][0-9]{0,3})\.[0-9]{2}")


def validate(inp):
    p = inp["prices"]
    if not isinstance(p, list) or not 1 <= len(p) <= 10**4:
        return "1 ≤ prices.length ≤ 10⁴"
    if not all(isinstance(x, str) and PRICE.fullmatch(x) for x in p):
        return "each price looks like 12.30 (0.00 to 9999.99)"
    return None
