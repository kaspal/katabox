## La idea

La partición de Lomuto de la [Ordenación rápida](learn:sorting/quick-sort) manda todos los valores iguales al pivote a un mismo lado, así que los duplicados la vuelven lenta. La **partición en tres partes** (el esquema de la "bandera holandesa") divide el tramo en tres: menores que el pivote, iguales y mayores. El bloque de iguales queda terminado en una sola pasada. Aquí el pivote es el primer valor, y tres punteros delimitan las partes: `a[lo..lt-1]` son los menores, `a[lt..i-1]` los iguales, `a[i..gt]` lo que falta por revisar y `a[gt+1..hi]` los mayores.

### La partición de Hoare

El esquema original de Hoare (que aquí no se anima) mueve dos índices hacia dentro desde los extremos del tramo. El de la izquierda para en un valor que no sea menor que el pivote y el de la derecha en uno que no sea mayor; se intercambia la pareja y los dos siguen, hasta que se cruzan. Cada intercambio coloca dos valores a la vez, así que intercambia menos que Lomuto, y como los dos índices paran en los valores iguales, los duplicados se reparten entre los lados. Eso sí, el pivote no acaba necesariamente en su lugar definitivo: el tramo se divide donde se cruzaron los índices y los dos lados se vuelven a ordenar.

## Pasos

1. Toma `pivot = a[lo]`; pon `lt = lo`, `i = lo + 1`, `gt = hi`.
2. Mientras `i <= gt`, mira `a[i]`.
3. Menor: intercámbialo con `a[lt]` y avanza tanto `lt` como `i`.
4. Mayor: intercámbialo con `a[gt]` y retrocede `gt`. `i` se queda, porque el valor que llega desde `gt` está sin revisar.
5. Igual: simplemente avanza `i`.
6. Ordena `lo..lt-1` y `gt+1..hi` de la misma manera; `lt..gt` ya está terminado.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n)` (todos los valores iguales: una pasada) |
| Caso medio | `O(n log n)` |
| Peor caso | `O(n²)` (entrada ordenada o invertida, con el primer valor como pivote) |
| Espacio extra | `O(log n)` de pila en promedio, `O(n)` en el peor caso |
| Estable | No: los intercambios saltan por encima de valores iguales |
| En el sitio | Sí |

## Errores comunes

- **Avanzar `i` tras un intercambio con `gt`.** El valor que llega a `i` está sin revisar, así que hay que compararlo primero. Tras un intercambio con `lt`, el valor que llega a `i` viene del bloque de iguales (o es el mismo valor, si `lt` = `i`), así que ya está revisado e `i` avanza.
- **Entrada ordenada.** Con el primer valor como pivote, una entrada ordenada o invertida deja un lado vacío cada vez, igual que en Lomuto. Un pivote aleatorio lo soluciona.
- **Los duplicados son la clave.** Prueba "Pocos distintos": la recursión nunca baja más niveles que el número de valores distintos.

## Cuándo usarlo

Lomuto es el más fácil de escribir, pero es el que más intercambia y el que peor lleva los duplicados. Hoare es el que menos intercambia y va bien con valores casi todos distintos. La partición en tres partes gana cuando las claves se repiten mucho: notas, edades, categorías. Los tres solo cambian cómo se divide un tramo; la recursión alrededor es la misma.

## Temas relacionados

- [Ordenación rápida](learn:sorting/quick-sort): la partición de Lomuto, más sencilla pero débil con duplicados.
- [Selección rápida](learn:sorting/quickselect): una partición cada vez, quedándose solo con un lado.
- [Ordenación por conteo](learn:sorting/counting-sort): con pocos valores posibles, sin ninguna comparación.
