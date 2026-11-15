SELECT COALESCE(b.ref, k.ref) AS ref,
       b.amount_cents AS bank_cents,
       k.amount_cents AS books_cents
FROM bank b
FULL OUTER JOIN books k ON k.ref = b.ref
WHERE b.ref IS NULL OR k.ref IS NULL OR b.amount_cents <> k.amount_cents
ORDER BY COALESCE(b.ref, k.ref);
