## La idea

Un **palíndromo** se lee igual en los dos sentidos. Hacer crecer uno desde cada centro posible cuesta `O(n²)` en un texto como `aaaa…`. El **algoritmo de Manacher** reutiliza lo que ya encontraron los centros anteriores, así que el texto entero cuesta `O(n)`.

Primero se pone una barra entre las letras y alrededor de ellas: `aba` pasa a ser `|a|b|a|`. Ahora todo palíndromo tiene un centro en `s`: los impares sobre una letra y los pares sobre una barra, así que una sola regla sirve para los dos. `radius[i]` es hasta dónde llega a cada lado el palíndromo centrado en `i`. Gracias a las barras, también es la longitud de ese palíndromo en el texto original, que empieza en `(i - radius[i]) / 2`.

El algoritmo guarda el palíndromo que llega más a la derecha hasta ahora: su centro `center` y su borde `right`. Para una posición `i` dentro de él, el palíndromo refleja `i` sobre `2 · center - i`, cuyo radio ya se conoce. Se empieza por `min(right - i, radius[espejo])`: el valor del espejo, recortado en el borde, porque más allá de `right` no se ha comprobado nada. Después se **crece** mientras las letras de los dos lados coincidan, y se mueven `center` y `right` si el nuevo palíndromo llega más lejos. La salida es la longitud y el inicio del palíndromo más largo; en caso de empate gana el primero. El texto se escribe como una palabra, con letras de la a a la z.

### La entrada por defecto

Texto `abacabadab`, así que `s = |a|b|a|c|a|b|a|d|a|b|` con 21 posiciones. La `b` de `s[3]` crece hasta radio 3 (`aba`). La `c` de `s[7]` crece hasta radio 7: `abacaba`, con el borde en 14. En `s[11]`, la segunda `b` de ese palíndromo, el espejo es `s[3]`, así que empieza por `min(14 - 11, 3) = 3`; el par siguiente, `c` y `d`, no coincide, y deja de crecer enseguida. Más adelante, la `d` de `s[15]` crece hasta 5 (`badab`), que sigue siendo más corto. La salida es `[7, 0]`.

## Pasos

1. Construye `s` con barras y un arreglo `radius` de ceros; `center = right = 0`.
2. Para cada `i`: si `i < right`, empieza por `min(right - i, radius[2 · center - i])`.
3. Crece mientras existan los dos vecinos y sean iguales.
4. Si `i + radius[i] > right`, haz `center = i` y `right = i + radius[i]`.
5. Elige la primera `i` con el radio mayor; devuelve `[radius[i], (i - radius[i]) / 2]`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n)` |
| Espacio extra | `O(n)` para `s` y `radius` |

¿Por qué es lineal? Si el palíndromo del espejo termina estrictamente dentro del actual, no puede crecer: el primer par nuevo es el reflejo de un par que ya falló. Si no, cada paso de crecimiento con éxito empuja `right` una posición más allá, y `right` nunca retrocede, así que en total hay como mucho `2n + 1`.

## Errores comunes

- **Quitar las barras de los extremos.** Las barras convierten los palíndromos pares e impares en un solo caso, pero solo con una barra en cada extremo; sin ellas, el radio deja de ser la longitud en los palíndromos que tocan los bordes del texto.
- **No recortar en `right`.** El radio del espejo puede llegar más allá de la parte conocida; toma el mínimo.
- **Leer las posiciones de `s` como posiciones del texto.** Divide entre 2 para volver al texto.

## Cuándo usarlo

Úsalo para encontrar el trozo palíndromo más largo de un texto, o para conocer, en cada centro, el palíndromo que lo rodea: contar las subcadenas palíndromas es una suma sobre el arreglo de radios. El problema enlazado, El espejo más largo, pide justo esto.

## Temas relacionados

- [Función Z](learn:strings/z-function): la misma reutilización de la caja que más lejos llega, para prefijos.
- [Knuth-Morris-Pratt](learn:strings/kmp): otro recorrido lineal que nunca repite trabajo.
- [Dos punteros](learn:searching/two-pointers): crecer desde un centro con dos extremos que se mueven.
