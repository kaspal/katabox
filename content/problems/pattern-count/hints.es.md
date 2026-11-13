Comparar el patrón en cada posición puede costar `longitud del texto × longitud del patrón`, por ejemplo cuando los dos son casi todo `a`. Cuando una coincidencia parcial falla, ¿qué sabes ya del texto?
---
El texto que acabas de emparejar **es** un prefijo del patrón. Así que tras un fallo puedes seguir desde el prefijo más largo del patrón que también es sufijo de lo emparejado, en vez de empezar de cero.
---
La función de prefijos: `pi[i]` es la longitud del prefijo propio más largo de `pattern[0..i]` que también es su sufijo. Constrúyela para el patrón y recorre el texto llevando la longitud emparejada `k`, retrocediendo con `k = pi[k − 1]` en los fallos. Cada vez que `k` llega a la longitud del patrón, cuenta una y retrocede.
