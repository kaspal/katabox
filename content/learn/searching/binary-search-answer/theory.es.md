## La idea

A veces no hay ningún arreglo ordenado donde buscar, y aun así la búsqueda binaria funciona: buscas en el rango de **respuestas** posibles. Toma esta tarea: unos paquetes con pesos `weights` tienen que enviarse en su orden, la carga de un día no puede pasar de una capacidad y todo tiene que salir en `days` días. ¿Cuál es la menor capacidad que sirve?

Responder "¿basta la capacidad `c`?" es fácil: `days_needed` llena cada día con todo lo que cabe y cuenta los días. Encontrar directamente la menor `c` buena no lo es. La clave es que la respuesta de sí o no es **monótona**: si la capacidad `c` basta, cualquier capacidad mayor también. A lo largo de las capacidades posibles las respuestas dicen no, no, …, no, sí, sí, …, sí, y la búsqueda binaria encuentra el primer sí.

El rango es fácil de acotar. La capacidad tiene que ser al menos el paquete más pesado (tiene que caber en algún día), y el peso total siempre sirve (todo sale el primer día).

## Pasos

1. Pon `lo = max(weights)` y `hi = sum(weights)`.
2. Mientras `lo < hi`, prueba `mid = (lo + hi) // 2`.
3. Cuenta `days_needed(weights, mid)`.
4. Si es `<= days`, `mid` sirve y la respuesta es `mid` o menos: `hi = mid`.
5. Si no, `mid` se queda corta: `lo = mid + 1`.
6. Cuando `lo == hi`, devuelve `lo`: la menor capacidad que cabe.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n log S)` |
| Caso medio | `O(n log S)`: `log S` intentos, cada uno revisando los `n` paquetes |
| Peor caso | `O(n log S)` |
| Espacio extra | `O(1)` |

Aquí `S` es el ancho del rango de respuestas, `sum − max`. Probar todas las capacidades una a una costaría `O(n · S)`.

## Errores comunes

- **Una comprobación que no es monótona.** El método se basa en que "un sí sigue siendo sí". Si un valor mayor puede convertir un sí en un no, la búsqueda acaba en una respuesta equivocada sin avisar.
- **Un rango que deja fuera la respuesta.** Empezar por debajo de `max(weights)` hace que `days_needed` no tenga sentido (un paquete que nunca cabe). Asegúrate de que `hi` siempre sea un sí.
- **Errores de uno al final.** Para "el menor que sirve", conserva `mid` en un sí (`hi = mid`) y sáltalo en un no (`lo = mid + 1`). Para "el mayor que sirve", hazlo al revés y redondea `mid` hacia arriba para que el bucle no se atasque.

## Cuándo usarlo

Recurre a ella siempre que una tarea diga "encuentra el menor (o el mayor) valor tal que…" y comprobar un candidato sea sencillo: la velocidad más lenta que aún termina a tiempo, el trozo más largo que puedes cortar un número dado de veces, la menor suma máxima al dividir una lista en `k` partes. Escribe la comprobación, demuestra que es monótona, acota el rango y divide.

## Temas relacionados

- [Búsqueda binaria](learn:searching/binary-search): la misma división, sobre un arreglo ordenado.
- [Cota inferior y superior](learn:searching/lower-upper-bound): el primer sí es una cota inferior sobre una fila de síes y noes.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): por qué `log S` intentos ganan a `S`.
