## La idea

Para encontrar un patrón en un texto, la forma obvia prueba cada inicio y compara letra a letra, y tras cada fallo vuelve al inicio siguiente. Eso puede costar `O(n · m)`. **Knuth-Morris-Pratt** nunca retrocede en el texto: cuando llega un fallo, el propio patrón dice cuánto de lo ya emparejado se puede conservar.

La clave es un **borde**: un trozo que es a la vez prefijo propio y sufijo de una palabra. `abab` tiene el borde `ab`. La **tabla de fallos** guarda, para cada prefijo del patrón, la longitud de su borde propio más largo. Se construye aplicando la misma idea de emparejar al patrón contra sí mismo: `k` es la longitud del borde actual; si no coinciden, `k` cae a `fail[k - 1]`; si coinciden, crece en uno.

Después, el recorrido lee el texto una sola vez. `k` cuenta cuántas letras del patrón coinciden en este momento. Si hay un fallo, `k` baja a `fail[k - 1]`: ya se sabe que el borde de la parte emparejada coincide, así que se conserva. Cuando `k` llega a `m`, una coincidencia termina aquí; se apunta su inicio, `i - m + 1`, y se vuelve a saltar al borde, así que también se encuentran coincidencias **solapadas**. La salida es el número de coincidencias y después sus posiciones de inicio. El texto y el patrón se escriben como dos palabras, con letras de la a a la z.

### La entrada por defecto

Texto `abababcabab`, patrón `abab`. La tabla de fallos es `[0, 0, 1, 2]`: `a` y `ab` no tienen borde, `aba` tiene `a` y `abab` tiene `ab`.

El recorrido encuentra `abab` en la posición 0, conserva `k = 2` y completa una segunda coincidencia dos letras después, en la posición 2: las dos se solapan. En la `c`, el patrón espera una `a`: `k` cae de 2 a `fail[1] = 0`, y la `c` tampoco coincide. El recorrido vuelve a empezar y encuentra `abab` en la posición 7. La salida es `[3, 0, 2, 7]`.

## Pasos

1. Construye `fail`: para cada `i` desde 1, mientras `k > 0` y `pattern[i] ≠ pattern[k]`, haz `k = fail[k - 1]`; si coinciden, `k += 1`; guarda `fail[i] = k`.
2. Recorre el texto con `k = 0`: si no coinciden, retrocede igual; si coinciden, `k += 1`.
3. Cuando `k = m`, apunta `i - m + 1` y haz `k = fail[m - 1]`.
4. Devuelve el número de coincidencias y después las posiciones.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n + m)`: la tabla y después una pasada por el texto |
| Espacio extra | `O(m)` para la tabla de fallos |

Cada paso del recorrido o bien avanza a la letra siguiente o bien hace `k` más pequeño. `k` crece como mucho uno por letra, así que no puede caer más de `n` veces en total: el bucle interior suma `O(n)` en conjunto, no por cada letra.

## Errores comunes

- **Desfase de uno al retroceder.** Un fallo con `k` letras emparejadas usa `fail[k - 1]`, el borde de esas `k` letras, no `fail[k]`.
- **Volver a 0 tras una coincidencia.** Así se pierden las coincidencias solapadas; hay que caer al borde.
- **Empezar la tabla en 0.** `fail[0]` siempre vale 0, porque una sola letra no tiene borde propio; el bucle empieza en 1, o cada letra coincidiría consigo misma.

## Cuándo usarlo

Úsalo para contar o enumerar todas las apariciones de un patrón en un texto largo, con tiempo lineal garantizado. El problema enlazado, Contar el patrón, pide justo eso. La tabla de fallos por sí sola responde preguntas sobre bordes y periodos de una palabra.

## Temas relacionados

- [Función Z](learn:strings/z-function): otra búsqueda lineal, construida a partir de coincidencias con el prefijo.
- [Rabin-Karp](learn:strings/rabin-karp): la misma búsqueda con un hash rodante.
- [Aho-Corasick](learn:strings/aho-corasick): enlaces de fallo para muchos patrones a la vez.
- [Ventana deslizante](learn:searching/sliding-window): otra pasada única que nunca retrocede.
