-- Compares with NULL using =, which is never true: readers with no email at all are lost.
SELECT id, name, city
FROM readers
WHERE email = NULL OR trim(email) = ''
ORDER BY city, name, id;
