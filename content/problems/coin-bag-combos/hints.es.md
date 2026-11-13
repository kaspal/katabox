Construye cada bolsa moneda a moneda. Para no listar la misma bolsa dos veces en otro orden, añade solo monedas al menos tan grandes como la última.
---
Ordena las monedas. Una función recursiva `elegir(inicio, resto, bolsa)` prueba cada moneda desde el índice `inicio`, manteniendo la bolsa no decreciente.
---
Si `resto` es 0, anota una copia de la bolsa. Si no, para cada moneda `c = coins[i]` con `i ≥ inicio` y `c ≤ resto`, añádela, recurre con `inicio = i` (se puede reutilizar) y quítala. Probar las monedas en orden creciente da las bolsas ya en orden lexicográfico.
