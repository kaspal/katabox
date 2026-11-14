## La idea

**Unión-búsqueda** (o conjuntos disjuntos) sigue elementos repartidos en grupos que solo pueden fusionarse. Responde a dos operaciones:

- **union a b**: une el grupo de `a` con el grupo de `b`;
- **find x**: nombra el grupo de `x`, así que dos elementos están en el mismo grupo justo cuando find da la misma respuesta.

Cada grupo es un árbol, guardado en un solo arreglo: `parent[x]` es el elemento que hay encima de `x`, y la cima de cada árbol, la **raíz**, es su propio padre y da nombre al grupo. find sube por los enlaces de padre hasta la raíz; union encuentra las dos raíces y, si son distintas, cuelga una de la otra.

### Dos trucos

Dos trucos mantienen planos los árboles:

- **Unión por tamaño**: cuelga siempre la raíz del grupo menor de la raíz del mayor (si empatan, la raíz de `b` va bajo la de `a`). Así la altura de un árbol solo crece cuando su tamaño al menos se duplica, y se queda en `log n` como mucho.
- **Compresión de caminos**: cuando find ya ha encontrado la raíz, recorre otra vez el camino y apunta cada nodo directamente a la raíz. El siguiente find desde cualquiera de ellos da un solo paso.

Juntos hacen que cada operación cueste `O(α(n))` amortizado, donde `α` es la inversa de la función de Ackermann: menos de 5 para cualquier `n` que quepa en un ordenador, así que en la práctica es constante.

### La entrada por defecto

Seis elementos. union 0 1 y union 2 3 forman dos parejas, con raíces 0 y 2. union 1 3 junta dos grupos de tamaño 2, así que la raíz 2 queda bajo la raíz 0. union 4 5 empareja 4 y 5. union 3 5: find 3 sube 3 → 2 → 0 y comprime, apuntando 3 directamente a 0; después la raíz 4 (tamaño 2) va bajo la raíz 0 (tamaño 4). La salida son las raíces que dan find 5 y find 4, 0 y 0, y luego el número de grupos que quedan: 1.

## Pasos

1. Empieza con `parent[x] = x` y `size[x] = 1` para cada elemento.
2. find `x`: sigue los enlaces de padre hasta la raíz.
3. Después recorre otra vez desde `x` y pon la raíz como padre de cada nodo.
4. union `a b`: encuentra las dos raíces. Si son iguales, no hagas nada.
5. Si no, cuelga la raíz del grupo menor de la del mayor y suma los tamaños.

## Coste

| Operación | Coste |
|---|---|
| find, union | `O(α(n))` amortizado, constante en la práctica |
| find, union solo con unión por tamaño | `O(log n)` |
| Espacio extra | `O(n)`: los arreglos de padres y de tamaños |

## Errores comunes

- **Enlazar elementos en vez de raíces.** `parent[a] = b` solo mueve `a`, no su grupo, y puede romper los árboles. Enlaza siempre raíz con raíz.
- **Saltarte la unión por tamaño.** Sin ella, una serie de uniones puede construir una cadena larga, y el primer find sobre ella es `O(n)`.

## Cuándo usarlo

Usa unión-búsqueda siempre que los grupos se fusionen y preguntes "¿están conectados?": componentes conexas de un grafo que se construye arista a arista, detectar un ciclo a medida que llegan aristas, y Kruskal (próximamente), que construye un árbol de expansión mínima.

## Temas relacionados

- [Diámetro de un árbol](learn:trees/tree-diameter): otro árbol guardado como enlaces de padre.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): cómo se comparan `log n` y casi constante.
- Kruskal (próximamente): unión-búsqueda elige las aristas de un árbol de expansión mínima.
