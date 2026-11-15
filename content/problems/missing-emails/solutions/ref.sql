SELECT id, name, city
FROM readers
WHERE email IS NULL OR trim(email) = ''
ORDER BY city, name, id;
