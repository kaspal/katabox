## La idea

La búsqueda lineal es la forma más sencilla de encontrar un valor: mira la primera posición, luego la siguiente, y así hasta dar con el objetivo o quedarte sin valores. No le pide nada a los datos. Los valores pueden estar en cualquier orden, repetirse o llegar de uno en uno desde un archivo. El precio es que, en el peor caso, se mira cada valor una vez.

El ejemplo devuelve la **primera** posición que contiene el objetivo, porque para en la primera coincidencia. Si el objetivo no está, devuelve `-1`, una posición que ningún arreglo tiene.

## Pasos

1. Empieza en `i = 0`.
2. Compara `a[i]` con `target`.
3. Si son iguales, devuelve `i`.
4. Si no, pasa a `i + 1` y vuelve al paso 2.
5. Si `i` se sale de la última posición, devuelve `-1`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(1)` (el objetivo está en la posición 0) |
| Caso medio | `O(n)`: unas `n / 2` comprobaciones si el objetivo está dentro |
| Peor caso | `O(n)` (el objetivo es el último o no está: se miran los `n` valores) |
| Espacio extra | `O(1)`: un índice |

## Errores comunes

- **Olvidar el caso en que no está.** Si el bucle termina sin coincidencia, la función tiene que devolver algo igualmente. Devolver `0` o nada parecería una respuesta real; `-1` no se confunde con ninguna posición.
- **Devolver demasiado tarde.** Para obtener la primera coincidencia, devuelve en cuanto la encuentres. Un bucle que sigue y recuerda el último `i` que coincidió devuelve la última copia.
- **Buscar en los mismos datos una y otra vez.** Una búsqueda lineal es barata. Miles de ellas sobre los mismos valores cuestan `O(n)` cada una; ordenar una vez y usar búsqueda binaria, o construir una tabla hash, compensa enseguida.

## Cuándo usarlo

Úsala cuando los datos son pocos, están desordenados o se leen una sola vez: una lista de una docena de ajustes, las líneas de un archivo que llegan en orden o una comprobación que se hace una única vez. También es la opción correcta cuando necesitas la primera coincidencia por posición, o cuando la prueba es algo más que igualdad (la primera palabra de más de diez letras, la primera lectura que falla). Para consultas repetidas en datos grandes, ordena y busca, o usa una tabla hash.

## Temas relacionados

- [Búsqueda binaria](learn:searching/binary-search): `O(log n)` comprobaciones, una vez ordenados los valores.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): cómo se comparan `n` comprobaciones con `log n`.
- Tablas hash (próximamente): consultas en `O(1)` de media, en cualquier orden.
