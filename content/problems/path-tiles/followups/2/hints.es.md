En el problema base, la cuenta para la longitud `i` solo necesitaba las cuentas de
longitudes menores. Ahora, antes de poner una losa larga, tienes que saber en qué termina
el pavimento hasta ese punto.
---
Divide la cuenta de cada longitud en dos: pavimentos que terminan en losa larga y
pavimentos que no (cuenta el pavimento vacío entre los segundos).
---
Un pavimento de longitud `i` que termina en losa larga es un pavimento de longitud `i − 3`
que no termina en una, más esa losa. Un pavimento que no termina en losa larga es
cualquier pavimento de longitud `i − 1` más una cuadrada, o cualquiera de longitud `i − 2`
más una losa corta. La respuesta es la suma de los dos tipos para la longitud `n`.
