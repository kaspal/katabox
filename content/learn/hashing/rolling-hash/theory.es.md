## La idea

Para comparar todas las ventanas de `length` letras de un texto, calcular el hash de cada ventana desde cero cuesta `O(length)` por ventana. Un **hash rodante** obtiene el hash de la ventana siguiente a partir del anterior en `O(1)`: quita la letra que sale, desplaza y añade la letra que entra.

El truco es un **hash polinómico**. Lee la ventana como un número en base 26, con las letras como dígitos (`a = 0`, `b = 1`, ..., `z = 25`), y mantenlo pequeño con `mod 101`. Para la ventana `abc`:

`h = (0 · 26² + 1 · 26 + 2) mod 101 = 28`

El **peso** de cada letra depende de su posición: la primera letra de una ventana de 3 vale `26² mod 101 = 70`.

### Rodar un paso

Para pasar de `abc` a `bca`:

1. **Quita** la `a` inicial: resta su código por su peso, `0 · 70`. h sigue en 28.
2. **Desplaza**: multiplica por 26, y cada letra que queda sube una posición.
3. **Añade** la nueva `a`: suma su código. `h = (28 · 26 + 0) mod 101 = 21`.

Sea cual sea `length`, son un número constante de operaciones. El texto por defecto `abcabcab` con `length` 3 tiene 6 ventanas, y sus hashes son 28, 21, 40, 28, 21, 40: ventanas iguales siempre tienen hashes iguales.

### Colisiones

Lo contrario no es cierto. Solo hay 101 hashes posibles y muchísimas más ventanas posibles, así que dos ventanas distintas pueden compartir hash. Hashes iguales significan "quizá iguales": compara las letras para estar seguro. El código real usa un módulo primo grande (a menudo alrededor de 10⁹) para que las colisiones sean raras. Rabin–Karp (próximamente) construye sobre esto una búsqueda completa de subcadenas.

## Pasos

1. Calcula `power = 26^(length−1) mod 101`, el peso de la primera letra de una ventana.
2. Calcula el hash de la primera ventana letra a letra: `h = (h · 26 + code) mod 101`.
3. Para cada letra siguiente: resta `code · power` de la letra que sale, manteniendo `h` en `0..100`.
4. Luego `h = (h · 26 + code) mod 101` con el código de la letra que entra, y devuelve `h`.

## Coste

| Operación | Coste |
|---|---|
| Primera ventana | `O(length)` |
| Cada ventana siguiente | `O(1)` |
| Todas las ventanas de un texto de `n` letras | `O(n)` |
| Espacio extra | `O(1)` aparte de la salida |

## Errores comunes

- **Restos negativos.** Tras restar, `h` puede quedar por debajo de 0. El `%` de Python devuelve un resultado no negativo, pero el de JavaScript no; el ejemplo de TypeScript suma `MOD` antes de volver a aplicar `%`.
- **Fiarte de hashes iguales.** Una colisión hace que dos ventanas distintas parezcan iguales. Comprueba las letras cuando los hashes coincidan.
- **Usar la potencia equivocada.** El peso de la letra que sale es `base^(length−1)`, no `base^length`.

## Cuándo usarlo

Usa un hash rodante para comparar rápido muchas ventanas que se solapan: buscar un patrón en un texto, detectar una subcadena repetida o encontrar trozos duplicados en ficheros.

## Temas relacionados

- [Ventana deslizante](learn:searching/sliding-window): la misma actualización de "quita uno, añade uno".
- [Sumas prefijas](learn:foundations/prefix-sums): otra forma de obtener el valor de cualquier rango en `O(1)`.
- Rabin–Karp (próximamente): búsqueda de subcadenas con un hash rodante.
