-- Members who never came at all: a visit in October hides an idle November.
SELECT m.id, m.name
FROM members m
WHERE m.id NOT IN (SELECT member_id FROM visits)
ORDER BY m.name, m.id;
