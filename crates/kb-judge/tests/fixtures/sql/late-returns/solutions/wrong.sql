-- Counts a book returned on its due day as late.
SELECT m.name, l.title,
       CAST(julianday(l.returned) - julianday(l.due) AS INTEGER) AS days_late
FROM loans l
JOIN members m ON m.id = l.member_id
WHERE l.returned >= l.due
ORDER BY days_late DESC, l.title;
