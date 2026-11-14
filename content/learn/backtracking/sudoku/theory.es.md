## La idea

Una cuadrícula de sudoku de tamaño `s` (4 o 9) se divide en cajas de `√s × √s`. Hay que llenar cada casilla vacía con una cifra de 1 a `s` de modo que ninguna fila, columna o caja tenga una cifra repetida. La vuelta atrás trata cada casilla vacía como una elección y avanza a base de tanteos, deshaciendo un tanteo en cuanto no lleva a ninguna parte.

La función `fill()` del ejemplo busca la **primera casilla vacía en orden de lectura** (fila a fila, de izquierda a derecha). Si no hay ninguna, la cuadrícula está resuelta. Si la hay, prueba las cifras **en orden creciente**; una cifra vale cuando `ok` no la encuentra en la fila, la columna ni la caja de la casilla. Escribe la cifra y vuelve a llamar a `fill()`. Si esa llamada informa de éxito, el éxito se pasa directamente hacia arriba. Si no, la casilla vuelve a quedar vacía (**deshacer**) y se prueba la cifra siguiente. Cuando ninguna cifra cabe, `fill()` informa de fracaso, y la casilla anterior pasa a su siguiente cifra. La salida es la cuadrícula resuelta fila a fila, o `[-1]` cuando los datos iniciales no tienen solución. La cuadrícula se escribe fila a fila, con las filas separadas por `/` y `.` para una casilla vacía; también hay un ejemplo de 9×9. Los datos iniciales se muestran en negrita.

### La entrada por defecto

La cuadrícula de 4×4 `1.../..2./.3../...4`. Fila 0: la casilla (0, 1) recibe un 2, y la casilla (0, 2) toma el 3, la primera cifra que cabe. Entonces la casilla (0, 3) necesita un 4, pero la columna 3 ya tiene un 4: callejón sin salida. Se deshace el 3, (0, 2) pasa a 4 y (0, 3) a 3. La fila 1 se llena con 3, 4 y 1 alrededor de su 2 inicial. En la fila 2, (2, 0) recibe primero un 2 y (2, 2) un 1, pero (2, 3) volvería a necesitar un 4: callejón sin salida. Ninguna otra cifra cabe en (2, 2), así que la búsqueda vuelve a (2, 0), que pasa a 4; ahora (2, 2) es 1 y (2, 3) es 2. La fila 3 termina con 2, 1 y 3.

En total se colocan 15 cifras y se deshacen 3. La salida es `[1, 2, 4, 3, 3, 4, 2, 1, 4, 3, 1, 2, 2, 1, 3, 4]`.

## Pasos

1. Busca la primera casilla vacía; si no hay ninguna, informa de éxito.
2. Para cada cifra de 1 a `s`: si su fila, su columna y su caja no la tienen, escríbela y llena el resto.
3. Si el resto sale bien, informa de éxito; si no, vacía la casilla y prueba la cifra siguiente.
4. Si ninguna cifra funciona, informa de fracaso para que la casilla anterior cambie su tanteo.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | Rápido con pocos huecos: cada uno se llena tras unas pocas comprobaciones |
| Caso medio | Exponencial en los huecos: cada casilla vacía puede multiplicar los tanteos |
| Peor caso | Exponencial en los huecos, hasta `s` tanteos por casilla vacía |
| Espacio extra | `O(casillas)`: la copia de la cuadrícula y una llamada por casilla vacía |

## Errores comunes

- **No vaciar la casilla al deshacer.** Una cifra olvidada sigue bloqueando su fila, su columna y su caja, y la búsqueda de la primera casilla vacía se la salta.
- **Perder el éxito.** En cuanto una llamada más profunda ha resuelto la cuadrícula, vuelve enseguida; probar más cifras desharía la solución.
- **La esquina de la caja.** La caja empieza en `r − r % b`, `c − c % b` para un lado de caja `b`, no en `r / b`.
- **Fiarse de los datos iniciales.** El ejemplo nunca compara dos datos iniciales entre sí; un resolvedor cuidadoso los comprueba antes de buscar.

## Cuándo usarlo

Úsalo para rompecabezas en los que las casillas deben tomar valores con reglas por fila, columna o grupo: sudokus, horarios con incompatibilidades, colorear un mapa pequeño. Los resolvedores de verdad añaden dos ideas. **Elegir la casilla más restringida**, la casilla vacía con menos cifras permitidas, en vez de la primera: así los callejones sin salida aparecen mucho antes. Y **propagar restricciones**: tras cada colocación, quitar esa cifra de las opciones de sus vecinas y llenar cualquier casilla que se quede con una sola opción, lo que a menudo resuelve cuadrículas fáciles sin tantear nada.

## Temas relacionados

- [N reinas](learn:backtracking/n-queens): colocar, comprobar y quitar, una fila cada vez.
- [Sopa de letras](learn:backtracking/word-search): vuelta atrás sobre las casillas de una cuadrícula.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): una máscara de bits por fila, columna y caja hace la comprobación instantánea.
- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): el mismo orden de bajar hasta el fondo y volver.
