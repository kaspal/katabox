## La idea

Pega el patrón y el texto con una barra entre ellos: `pattern|text`. Para cada posición `i` de esa cadena `s`, `z[i]` es la longitud del trozo más largo que empieza en `i` y coincide con el principio de `s`. La barra no es una letra, así que ninguna coincidencia puede atravesarla: `z[i]` vale como mucho `m`, la longitud del patrón, y una posición del texto con `z[i] = m` es una coincidencia completa. Su inicio en el texto es `i - m - 1`.

Calcular cada `z[i]` comparando desde cero puede costar `O(n · m)`. El truco es la **caja Z** `[left, right)`: la coincidencia que llega más a la derecha hasta ahora. Dentro de ella, `s[left..right)` es una copia de `s[0..right - left)`. Así, una posición `i` dentro de la caja se parece a la posición `i - left` cerca del principio, cuyo valor ya se conoce. Se empieza por `min(right - i, z[i - left])`: el valor reflejado, recortado en el borde de la caja, porque nada más allá de `right` se ha comprobado todavía. Después se **extiende** letra a letra, y la caja se mueve si la coincidencia llega ahora más a la derecha. `z[0]` se queda en 0; nunca se lee. La salida es el número de coincidencias y después sus posiciones. El texto y el patrón se escriben como dos palabras, con letras de la a a la z.

### La entrada por defecto

Texto `aabaabaab`, patrón `aab`, así que `s = aab|aabaabaab`. `z[1] = 1` (`a`, y después la `b` no coincide con la `a`) y `z[4] = 3`: `aab`, y después la barra lo detiene. La caja pasa a ser `[4, 7)`. En `i = 5`, la caja copia `z[1] = 1`, y una comparación más falla. En `i = 6`, copia `z[2] = 0`. En `i = 7`, fuera de la caja, extender vuelve a dar 3, y lo mismo pasa en `i = 10`. Las posiciones con `z = 3` son 4, 7 y 10, así que la salida es `[3, 0, 3, 6]`.

## Pasos

1. Construye `s = pattern + "|" + text` y un arreglo `z` de ceros, con `left = right = 0`.
2. Para cada `i` desde 1: si `i < right`, empieza por `min(right - i, z[i - left])`.
3. Mientras `s[z[i]]` sea igual a `s[i + z[i]]`, aumenta `z[i]`.
4. Si `i + z[i] > right`, haz `left = i` y `right = i + z[i]`.
5. Cada `i` tras la barra con `z[i] = m` es una coincidencia en `i - m - 1`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n + m)`: cada extensión con éxito empuja `right` hacia delante |
| Espacio extra | `O(n + m)` para `s` y `z` |

Una comparación que tiene éxito al extender mueve `right` una posición más allá, y `right` nunca retrocede, así que hay como mucho `n + m`; cada posición añade como mucho una comparación fallida.

## Errores comunes

- **Un separador que puede aparecer en el texto.** Entonces las coincidencias podrían atravesarlo y `z` podría superar `m`. Sirve cualquier símbolo que no sea una letra.
- **No recortar en el borde de la caja.** El valor reflejado puede pasar de `right`, donde no se sabe nada; toma el mínimo.
- **Extremo derecho exclusivo.** Aquí `right` está una posición después de la caja, así que la comprobación es `i < right`.

## Cuándo usarlo

Úsala para buscar un solo patrón, o para preguntas sobre prefijos: hasta dónde coincide cada posición con el principio, el periodo más corto de una palabra. Frente a [Knuth-Morris-Pratt](learn:strings/kmp), encuentra las mismas coincidencias en el mismo tiempo, con un solo arreglo calculado sobre patrón y texto juntos en lugar de una tabla más un recorrido. El problema enlazado, Contar el patrón, se resuelve con cualquiera de los dos.

## Temas relacionados

- [Knuth-Morris-Pratt](learn:strings/kmp): bordes en lugar de coincidencias con el prefijo.
- [Palíndromo más largo de Manacher](learn:strings/manacher): la misma reutilización de una caja, alrededor de palíndromos.
- [Rabin-Karp](learn:strings/rabin-karp): búsqueda calculando el hash de las ventanas.
