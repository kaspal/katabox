## Enfoque: vuelta atrás sobre bolsas no decrecientes

Ordena `coins`. Explora las bolsas con `elegir(inicio, resto)`:

- si `resto == 0`, la bolsa actual paga exacto: anota una copia;
- si no, para `i` desde `inicio`, mientras `coins[i] ≤ resto`: añade `coins[i]`, llama a `elegir(i, resto − coins[i])` y quítala.

Pasar `i` (no `i + 1`) permite repetir una moneda; no volver nunca a monedas menores hace que cada bolsa sea no decreciente, así que cada bolsa aparece exactamente una vez. Como en cada posición se prueban las monedas en orden creciente, las bolsas salen en orden lexicográfico: no hace falta ordenar al final.

Parar en cuanto `coins[i] > resto` (las monedas están ordenadas) poda la búsqueda.

**Complejidad:** proporcional al número de bolsas parciales exploradas, pequeño con estos límites (target ≤ 60 y como mucho 1000 respuestas).
