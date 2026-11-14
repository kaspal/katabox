## La idea

Dada una cuadrícula de letras y una palabra, ¿se puede escribir la palabra con un **camino** que pase de una casilla a otra vecina (arriba, derecha, abajo o izquierda, nunca en diagonal) sin usar ninguna casilla dos veces? La vuelta atrás alarga el camino letra a letra y retrocede cada vez que la letra siguiente no se puede alcanzar.

El ejemplo prueba cada casilla como inicio, en orden de lectura. `walk(r, c, k)` pregunta si el camino puede seguir desde la casilla `(r, c)` como letra `k` de la palabra. Falla enseguida si la letra de la casilla no es `word[k]` o si la casilla ya está en el camino. Si no, **se pisa** la casilla (se añade al camino). Si era la última letra, la palabra está encontrada. Si no, se prueban las cuatro vecinas en el orden arriba, derecha, abajo, izquierda, cada una como letra `k + 1`. Cuando ninguna lleva hasta el final, se **retrocede** y la casilla sale del camino (se quita), y la llamada falla, así que quien la hizo prueba su siguiente dirección. Gana el primer camino completo.

La salida es `[1, y después los pares (fila, columna) del camino]` cuando se encuentra la palabra, o `[0]` cuando no. La entrada se escribe como un mapa de letras, con las filas separadas por `/`, y la palabra, letras de la a a la z. El tablero resalta el camino actual.

### La entrada por defecto

El mapa `wqzk/arym/tels/bnow` y la palabra `water`:

```
w q z k
a r y m
t e l s
b n o w
```

El inicio `(0, 0)` tiene una `w`. Arriba está fuera, a la derecha hay una `q` y abajo una `a`: se pisa. Desde `(1, 0)`, arriba está la `w` (letra equivocada), a la derecha una `r` y abajo una `t`: se pisa. Desde `(2, 0)`, arriba está la `a` y a la derecha una `e`: se pisa. Desde `(2, 1)`, arriba hay una `r`, la última letra. La palabra baja, gira a la derecha y vuelve a subir sin retroceder ni una vez, y la segunda `w`, en `(3, 3)`, ni se llega a probar.

La salida es `[1, 0, 0, 1, 0, 2, 0, 2, 1, 1, 1]`. El ejemplo "Comienzos falsos" muestra la otra cara: varios inicios prometedores que acaban todos en callejones sin salida.

## Pasos

1. Para cada casilla en orden de lectura, llama a `walk(r, c, 0)`; detente en el primer éxito.
2. En `walk(r, c, k)`: falla si la letra no es `word[k]` o si la casilla está en el camino.
3. Añade la casilla. Si `k` es la última posición, éxito.
4. Prueba las vecinas arriba, derecha, abajo, izquierda con `k + 1`; éxito si alguna lo consigue.
5. Si no, quita la casilla y falla.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(R · C)`: el recorrido por los inicios cuando la primera letra casi nunca coincide |
| Caso medio | `O(R · C · 3ᴸ)`: tras el primer paso, como mucho 3 direcciones nuevas por letra |
| Peor caso | `O(R · C · 3ᴸ)`, en cuadrículas llenas de letras repetidas |
| Espacio extra | `O(L)` para el camino y la profundidad de llamadas |

## Errores comunes

- **Olvidar retroceder.** Una casilla que se queda en el camino bloquea todas las ramas posteriores que la necesiten.
- **Reutilizar una casilla.** Sin la comprobación del camino, una palabra como `aba` aparecería en una cuadrícula que solo tiene `ab`.
- **Comprobación lenta del camino.** El ejemplo recorre el camino para cada casilla; una cuadrícula de marcas "en el camino" hace la comprobación en tiempo constante.

## Cuándo usarlo

Úsalo para seguir una palabra, una ruta o una forma por una cuadrícula. Con **muchas** palabras, buscar cada una por separado repite el mismo trabajo. En su lugar, guarda todas las palabras en un [trie](learn:trees/trie) y recorre la cuadrícula una sola vez desde cada inicio, siguiendo los hijos del trie: una rama se detiene en cuanto ninguna palabra empieza por las letras reunidas. El problema enlazado, Caza de palabras, necesita esa idea.

## Temas relacionados

- [Trie (árbol de prefijos)](learn:trees/trie): muchas palabras, un solo recorrido.
- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): el mismo recorrido de bajar hasta el fondo y volver, en un grafo.
- [Sudoku](learn:backtracking/sudoku): vuelta atrás sobre las casillas de una cuadrícula.
- [Búsqueda en anchura en una cuadrícula](learn:grid-pathfinding/grid-bfs): la ruta más corta por una cuadrícula en vez de una palabra exacta.
