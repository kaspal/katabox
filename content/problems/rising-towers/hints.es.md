Sea `best[i]` la elección creciente más larga que termina en la torre `i`. ¿Cómo se relaciona con las torres anteriores a `i`? Eso da una solución O(n²), demasiado lenta aquí.
---
En su lugar, para cada longitud `k`, guarda la menor altura posible de la última torre de una elección creciente de longitud `k`. ¿Por qué esa lista es estrictamente creciente?
---
Recorre las torres. Para la altura `h`, busca la primera entrada de esa lista que sea `≥ h` (búsqueda binaria) y sustitúyela por `h`; si no hay ninguna, añade `h`. La longitud final de la lista es la respuesta.
