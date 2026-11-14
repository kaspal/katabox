## La idea

Con valores ordenados y un `target`, busca dos posiciones cuyos valores sumen eso. Comprobar todos los pares cuesta `O(n²)`. Dos punteros lo hacen en una sola pasada: `i` empieza en el valor más pequeño, `j` en el más grande, y miras `a[i] + a[j]`.

- Si la suma es el objetivo, has terminado.
- Si se queda **corta**, `a[i]` no sirve: ni siquiera con el mayor valor que sigue en juego, `a[j]`, llega, y cualquier otra pareja es menor. Así que `i` avanza a la derecha.
- Si se **pasa**, `a[j]` no sirve por la razón contraria: incluso con la menor pareja que queda, `a[i]`, se pasa. Así que `j` retrocede a la izquierda.

### Por qué no se salta ningún par

Cada movimiento quita un valor que no puede formar un par válido con los valores que siguen entre los punteros. Así que, si existe un par válido, sus dos posiciones siguen dentro de `i..j` hasta que los punteros caen justo en ellas. Cuando `i` y `j` se encuentran, todos los valores están descartados, y la respuesta es `[-1, -1]`.

## Pasos

1. Pon `i = 0` y `j = len(a) - 1`.
2. Mientras `i < j`, calcula `total = a[i] + a[j]`.
3. Si `total == target`, devuelve `[i, j]`.
4. Si `total < target`, mueve `i` un lugar a la derecha.
5. Si no, mueve `j` un lugar a la izquierda.
6. Cuando los punteros se encuentran, devuelve `[-1, -1]`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(1)` (el par de los extremos es la respuesta) |
| Caso medio | `O(n)` |
| Peor caso | `O(n)`: cada paso mueve un puntero, como mucho `n − 1` pasos |
| Espacio extra | `O(1)`: dos índices |

Si los valores aún no están ordenados, ordenarlos antes añade `O(n log n)`.

## Errores comunes

- **Entrada sin ordenar.** Todo el razonamiento se basa en que "moverse a la derecha hace la suma mayor". Sin orden, mover un puntero puede saltarse la respuesta.
- **Usar `i <= j`.** Cuando los punteros se encuentran apuntan al mismo valor, y un valor no puede emparejarse consigo mismo. Para en `i < j`.
- **Ordenar pierde las posiciones originales.** Si la tarea pide posiciones de la lista original, sin ordenar, ordena pares de (valor, posición) o usa una tabla hash.

## Cuándo usarlo

Úsalos para preguntas sobre pares en datos ordenados: dos valores con una suma o diferencia dada, el par más cercano a un objetivo o tres valores con una suma dada (fija uno y aplica dos punteros al resto, `O(n²)` en total). La misma forma de "un puntero en cada extremo, mueve el que no puede ayudar" también mezcla listas ordenadas y comprueba palíndromos.

## Temas relacionados

- [Ventana deslizante](learn:searching/sliding-window): dos punteros que avanzan los dos hacia delante.
- [Búsqueda binaria](learn:searching/binary-search): pares en `O(n log n)` buscando `target − a[i]`.
- [Ordenación por mezcla (Merge Sort)](learn:sorting/merge-sort): su mezcla recorre dos tramos con dos punteros.
- Tablas hash (próximamente): sumas de pares en `O(n)` sin ordenar.
