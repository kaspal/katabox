-- Ignores the preference: the phone always wins.
SELECT id,
       last_name || ', ' || first_name || ': ' ||
       COALESCE(phone, email, 'no contact') AS contact_line
FROM clients
ORDER BY last_name, first_name, id;
