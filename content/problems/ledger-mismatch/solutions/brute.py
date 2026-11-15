"""References missing on one side or with different amounts, in plain Python."""


def solve(tables):
    bank = {ref: amount for _, ref, amount in tables["bank"]}
    books = {ref: amount for _, ref, amount in tables["books"]}
    rows = []
    for ref in sorted(set(bank) | set(books)):
        a, b = bank.get(ref), books.get(ref)
        if a is None or b is None or a != b:
            rows.append([ref, a, b])
    return {"columns": ["ref", "bank_cents", "books_cents"], "rows": rows}
