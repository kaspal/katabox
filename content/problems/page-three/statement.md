A second-hand marketplace shows its listings ten at a time, best rated first. Some listings are hidden by the moderators: the site never shows them, so they take no place on any page.

A visitor has clicked through to **page 3**. Sort the visible listings (`hidden = 0`) by `rating`, highest first, then by `title`, then by `id`, and return the listings on page 3: page 1 is the 1st to 10th visible listing, page 2 the 11th to 20th, so page 3 is the **21st to 30th**. Page 3 may hold fewer than ten listings, or none at all.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `listings(id, title, rating, hidden)`: `id` is the primary key; `title` is the listing's title; `rating` is a whole number of stars from 0 to 5; `hidden` is `1` for a listing the site does not show and `0` for one it shows.

## Result

Return the listings on page 3, with three columns:

| column | value |
| --- | --- |
| `id` | the listing's `id` |
| `title` | its title |
| `rating` | its rating |

The rows come in page order: `rating`, highest first, then `title`, then `id`. Titles compare as plain text, so `kettle` comes before `kettle coat`.

### Example 1

`listings` (25 rows, none hidden):

| id | title | rating | hidden |
| --- | --- | --- | --- |
| 18 | bike | 5 | 0 |
| 73 | lamp boots | 1 | 0 |
| 9 | radio kettle | 2 | 0 |
| 33 | radio | 3 | 0 |
| 16 | kettle desk | 2 | 0 |
| 64 | desk coat | 5 | 0 |
| 58 | kettle lamp | 3 | 0 |
| 61 | kettle boots | 5 | 0 |
| 84 | lamp boots | 4 | 0 |
| 49 | kettle coat | 1 | 0 |
| 27 | chair | 5 | 0 |
| 13 | radio bike | 4 | 0 |
| 63 | chair chair | 5 | 0 |
| 4 | lamp | 5 | 0 |
| 50 | boots | 5 | 0 |
| 56 | radio guitar | 5 | 0 |
| 78 | boots | 5 | 0 |
| 1 | boots | 1 | 0 |
| 93 | tent kettle | 2 | 0 |
| 35 | radio coat | 4 | 0 |
| 30 | radio tent | 1 | 0 |
| 76 | kettle | 5 | 0 |
| 14 | kettle | 3 | 0 |
| 41 | desk | 1 | 0 |
| 86 | lamp guitar | 1 | 0 |

Result:

| id | title | rating |
| --- | --- | --- |
| 41 | desk | 1 |
| 49 | kettle coat | 1 |
| 73 | lamp boots | 1 |
| 86 | lamp guitar | 1 |
| 30 | radio tent | 1 |

The ten listings rated 5 fill page 1. Page 2 holds the three rated 4, the three rated 3, the three rated 2 and the first listing rated 1 by title, `boots` (id 1). The other five listings rated 1 are the 21st to 25th, so page 3 has only five rows.

### Example 2

`listings` (12 rows, none hidden):

| id | title | rating | hidden |
| --- | --- | --- | --- |
| 8 | boots | 4 | 0 |
| 12 | kettle radio | 5 | 0 |
| 11 | bike | 3 | 0 |
| 47 | boots boots | 5 | 0 |
| 22 | lamp | 2 | 0 |
| 95 | chair chair | 5 | 0 |
| 86 | radio | 4 | 0 |
| 40 | tent coat | 3 | 0 |
| 33 | chair boots | 4 | 0 |
| 78 | radio | 3 | 0 |
| 28 | radio radio | 3 | 0 |
| 5 | radio lamp | 3 | 0 |

Result: no rows. Twelve listings end on page 2.

## Constraints

- 1 ≤ listings ≤ 1000; the ids are distinct, from 1 to 10⁶.
- A title is one to three lowercase words; several listings may share a title.
- `rating` is a whole number, 0 ≤ `rating` ≤ 5.
- `hidden` is `0` or `1`.
