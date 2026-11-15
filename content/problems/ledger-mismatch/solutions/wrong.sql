-- A left join only looks from the bank's side: payments only in the books are lost.
SELECT b.ref AS ref,
       b.amount_cents AS bank_cents,
       k.amount_cents AS books_cents
FROM bank b
LEFT JOIN books k ON k.ref = b.ref
WHERE k.ref IS NULL OR b.amount_cents <> k.amount_cents
ORDER BY b.ref;
