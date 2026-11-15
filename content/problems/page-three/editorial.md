## Idea

Paging is three steps, and their order matters:

1. **Filter** to the listings the site shows.
2. **Sort** them into one exact order, so every listing has a fixed position.
3. **Cut** out positions 21 to 30.

## The query

```sql
SELECT id, title, rating
FROM listings
WHERE hidden = 0
ORDER BY rating DESC, title, id
LIMIT 10 OFFSET 20;
```

## Details that matter

- **Page `p` skips `(p - 1) × 10` rows.** Page 3 skips the 20 rows of pages 1 and 2. Writing `OFFSET 30` skips three pages and returns page 4 instead; this off-by-one is the classic paging mistake.
- **`LIMIT` and `OFFSET` apply after `WHERE` and `ORDER BY`.** Hidden listings are removed before counting, so they never take a slot, and the cut is made on the sorted list, not on the table's storage order.
- **The order must be total.** If two listings share a rating and a title, only the `id` decides which page each lands on. Without it, a listing could appear on two pages or on none as the database chooses a different order between requests.
- **Short or empty pages are fine.** `LIMIT 10` is an upper bound: with 25 visible listings page 3 has 5 rows, and with 20 or fewer it has none.

## Cost

SQLite scans the listings and sorts the visible ones. Because of the `LIMIT`, it only needs to keep the best 30 rows while sorting, but it still reads every row. Deep pages get slower with `OFFSET`, since the skipped rows are still produced and thrown away.
