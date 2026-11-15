SELECT id,
       last_name || ', ' || first_name || ': ' ||
       COALESCE(CASE prefers WHEN 'phone' THEN phone WHEN 'email' THEN email END,
                phone, email, 'no contact') AS contact_line
FROM clients
ORDER BY last_name, first_name, id;
