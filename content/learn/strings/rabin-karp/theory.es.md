## La idea

Comparar el patrón con cada ventana del texto letra a letra cuesta hasta `m` comparaciones por ventana. **Rabin-Karp** compara primero números: calcula el hash del patrón una vez, mantiene un [hash rodante](learn:hashing/rolling-hash) de la ventana actual y solo mira las letras cuando los dos hashes son iguales.

El hash lee una palabra como un número en base 26, con `a = 0` hasta `z = 25`, y lo mantiene pequeño con `mod 101`. La primera letra de una ventana de `m` letras pesa `high = 26^(m - 1) mod 101`. Avanzar una posición a la derecha cuesta `O(1)`: resta el código de la letra que sale por `high`, multiplica por 26, suma el código de la letra que entra y vuelve a tomar el resto.

Palabras iguales siempre tienen hashes iguales, pero solo existen 101 hashes, así que palabras distintas pueden compartir uno. Hashes iguales significan "quizá": deciden las letras. Una coincidencia real se apunta; una falsa alarma es una **colisión**, y el ejemplo las cuenta aparte para que puedas verlas. La salida es el número de coincidencias, el número de colisiones y después las posiciones de las coincidencias. El texto y el patrón se escriben como dos palabras, con letras de la a a la z.

### La entrada por defecto

Texto `bcaxaavbcab`, patrón `bca`. El hash del patrón es `(1 · 26 + 2) · 26 + 0 = 728`, y `728 mod 101 = 21`. Las 9 ventanas tienen hashes 21, 62, 93, 95, 21, 42, 84, 21, 40.

Tres ventanas dan 21. En la posición 0 las letras son `bca`: una coincidencia. En la posición 4 la ventana es `aav`: `0 · 676 + 0 · 26 + 21 = 21` también, pero las letras son distintas, así que es una colisión. En la posición 7 las letras vuelven a ser `bca`. La salida es `[2, 1, 0, 7]`.

## Pasos

1. Si el patrón es más largo que el texto, devuelve `[0, 0]`.
2. Calcula `high`, el hash del patrón y el hash de la primera ventana.
3. Para cada ventana: si los hashes son iguales, compara las letras y cuenta una coincidencia o una colisión.
4. Si hay una letra después de la ventana, rueda: `((window - code(sale) · high) · 26 + code(entra)) mod 101`.
5. Devuelve las coincidencias, las colisiones y después las posiciones.

## Coste

| Caso | Coste |
|---|---|
| Mejor | `O(n + m)`: los hashes casi nunca coinciden |
| Promedio | `O(n + m)` esperado, con un buen módulo |
| Peor | `O(n · m)` cuando muchas ventanas colisionan o coinciden, porque cada una cuesta `m` comparaciones |
| Espacio extra | `O(1)` aparte de la salida |

## Errores comunes

- **Fiarte del hash.** Saltarte la comprobación de las letras convierte cada colisión en una respuesta incorrecta.
- **Restos negativos.** En TypeScript, `%` puede dar negativo tras la resta; el ejemplo suma `MOD` antes de volver a aplicar `%`.
- **Un módulo diminuto.** 101 mantiene los números legibles aquí y hace visibles las colisiones. Un módulo primo grande (alrededor de 10⁹) hace muy improbable una falsa alarma en un texto aleatorio, y el **doble hash**, dos hashes con módulos distintos que tienen que coincidir los dos, la hace aún más rara. Menos colisiones significan menos comparaciones desperdiciadas, así que el tiempo esperado sigue siendo lineal.

## Cuándo usarlo

Úsalo cuando los hashes de las ventanas sirven por sí mismos: comparar muchas ventanas de la misma longitud, buscar a la vez varios patrones de una misma longitud o encontrar un trozo que aparece dos veces. El problema enlazado, El trozo repetido, busca así una ventana repetida.

## Temas relacionados

- [Hash rodante](learn:hashing/rolling-hash): el hash de ventana por sí solo.
- [Knuth-Morris-Pratt](learn:strings/kmp): búsqueda lineal sin hashes y sin casos malos.
- [Función Z](learn:strings/z-function): otra búsqueda lineal.
- [Ventana deslizante](learn:searching/sliding-window): la misma actualización de "quita uno, añade uno".
