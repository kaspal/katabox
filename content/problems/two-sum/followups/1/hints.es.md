Probar todos los pares es O(n²): con 10⁵ números son unos 5·10⁹ pares. ¿Puedes contar
las parejas de cada número sin mirarlas una a una?
---
Al llegar a `nums[j]`, cada posición anterior que contiene `target − nums[j]` forma un par
con ella.
---
Guarda un mapa de valor a cuántas veces lo has visto. Para cada número, suma la cuenta de
su pareja y después cuenta el propio número.
