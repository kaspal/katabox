## La idea

El **ancestro común más bajo** (LCA, por sus siglas en inglés) de dos nodos de un árbol con raíz es el nodo más profundo que tiene a los dos por debajo, contando cada nodo como ancestro de sí mismo. Subir de padre en padre cuesta `O(profundidad)` por pregunta. Los **saltos binarios** preparan una vez saltos de 1, 2, 4, 8, ... niveles, para que cada pregunta cueste `O(log n)`.

La tabla: `up[0][v]` es el padre de `v`, y la raíz 0 es su propio padre, así que un salto que se pasa de la cima se queda en la raíz. Después, `up[k][v] = up[k−1][up[k−1][v]]`: un salto de `2^k` niveles son dos saltos de `2^(k−1)`. La tabla tiene unas `log n` filas. La profundidad de cada nodo es la de su padre más 1; cada padre tiene un número menor que su hijo, así que una pasada en orden las rellena.

Una pregunta tiene dos fases. Primero, sube el nodo más profundo la diferencia de profundidad, un salto de la tabla por bit activo (una diferencia de 5 = 4 + 1 son dos saltos). Después, si los nodos aún son distintos, súbelos juntos, del salto más largo al más corto, pero solo cuando el salto los mantiene separados: un salto que los junta podría pasarse del LCA. Terminan como dos hijos distintos del LCA, cuyo padre es la respuesta.

### La entrada por defecto

Padres 0 0 1 1 2 2 3 4 6, para los nodos 1 a 9: los nodos 1 y 2 cuelgan de la raíz, 3 y 4 de 1, 5 y 6 de 2, y 7, 8 y 9 de 3, 4 y 6. Diez nodos dan 4 filas. Pareja (7, 8): los dos tienen profundidad 3. Sus ancestros 2 niveles más arriba son ambos 1, así que ese salto se omite; sus padres, 3 y 4, son distintos, así que los dos suben una vez, y la respuesta es el padre de 3: 1. Pareja (5, 9): 9 está un nivel más abajo y sube hasta 6. 6 y 5 son distintos, pero cada salto desde ellos cae en el mismo nodo, así que nada se mueve, y la respuesta es el padre de 6: 2. Pareja (3, 4): sus padres ya son el mismo, así que la respuesta es 1. La salida es 1, 2, 1.

## Pasos

1. Rellena la fila 0 con los padres (la raíz apunta a sí misma), y cada profundidad como la del padre más 1.
2. Rellena la fila `k` a partir de la `k − 1`: `up[k][v] = up[k−1][up[k−1][v]]`.
3. Para una pareja, haz que `a` sea el nodo más profundo y súbelo la diferencia de profundidad, un salto por bit activo.
4. Si `a` es igual a `b`, esa es la respuesta.
5. Si no, para `k` desde la fila más alta hasta 0, si `up[k][a]` es distinto de `up[k][b]`, sube los dos `2^k` niveles.
6. La respuesta es `up[0][a]`.

## Coste

| Operación | Coste |
|---|---|
| Construir la tabla | `O(n log n)` |
| Una pareja | `O(log n)` |
| Espacio extra | `O(n log n)` para la tabla |

## Errores comunes

- **Subir el nodo equivocado.** Intercambia primero para que `a` sea el más profundo; subir el menos profundo la diferencia se pasa.
- **Saltar cuando los ancestros coinciden.** Unos ancestros iguales pueden ser el LCA o estar por encima. Salta solo mientras sean distintos y toma el padre al final.
- **Pocas filas.** La tabla necesita filas suficientes para que el salto más largo cubra el nodo más profundo; `⌊log₂ n⌋ + 1` filas siempre bastan.

## Cuándo usarlo

Usa saltos binarios cuando un árbol fijo recibe muchas preguntas sobre ancestros: la distancia entre dos nodos (profundidad de `a` más profundidad de `b` menos dos veces la profundidad de su LCA), o el ancestro `k` niveles más arriba. Si el árbol cambia, hay que reconstruir la tabla.

## Temas relacionados

- [Diámetro de un árbol](learn:trees/tree-diameter): otro árbol guardado como enlaces de padre, medido con profundidades.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): leer los bits activos de la diferencia de profundidad.
- [Unión-búsqueda (conjuntos disjuntos)](learn:trees/union-find): de nuevo arreglos de padres, usados para fusionar grupos.
