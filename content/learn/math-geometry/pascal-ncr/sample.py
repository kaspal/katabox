def pascal_row(n):
    rows = [[1]]
    for i in range(1, n + 1):
        row = [1] * (i + 1)
        for k in range(1, i):
            row[k] = rows[i - 1][k - 1] + rows[i - 1][k]  # @step add
        rows.append(row)
    return rows[n]
