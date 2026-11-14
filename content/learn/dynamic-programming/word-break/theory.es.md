## La idea

Dado un texto sin espacios y una lista de palabras de diccionario, ¿se puede cortar el texto en trozos que sean **todos** palabras del diccionario? Cada palabra se puede usar las veces que haga falta. Probar todas las formas de cortar el texto duplica el trabajo con cada letra, y agarrar la palabra más larga que encaje puede llevar a un callejón sin salida.

La tabla hace una pregunta de sí o no sobre cada **prefijo**: `ok[end]` es verdadero cuando las primeras `end` letras se pueden partir en palabras. `ok[0]` es verdadero: el prefijo vacío no necesita palabras. Para cada `end` de 1 a `n`, el ejemplo prueba las palabras **en el orden dado**. Una palabra encaja si empezaría en `start = end - len(word) ≥ 0`, el prefijo anterior se puede partir (`ok[start]`) y las letras de `start` a `end` forman la palabra. La **primera** palabra que encaja pone `ok[end]` a verdadero y detiene la búsqueda para ese `end`. La salida es `[1]` si `ok[n]` es verdadero y `[0]` si no.

La entrada se escribe como el texto y después las palabras, cada una en su campo, con letras de la a a la z.

### La entrada por defecto

Texto `sunflowerpot`, palabras `sun`, `flow`, `flower`, `pot`, `er`. `sun` hace verdadero `ok[3]`. `flow` empieza en 3, así que `ok[7]` es verdadero. En `end = 9`, `flower` (también desde 3) encaja antes de probar `er`; `er` desde 7 también habría servido. `pot` empieza en 9, así que `ok[12]` es verdadero. Las celdas verdaderas son 0, 3, 7, 9 y 12, y la salida es `[1]`.

### Un callejón sin salida

El ajuste "un callejón sin salida" es `goldfishy` con `gold`, `fish`, `goldfish`, `shy`. `ok[4]` es verdadero (`gold`) y `ok[8]` también (`fish` desde 4, que se prueba antes que `goldfish`). `shy` sí encaja al final, pero empieza en 6, y `ok[6]` es falso. Todos los cortes se quedan sin salida en la `y` suelta: la salida es `[0]`.

Un partidor voraz que agarra la palabra más larga toma `goldfish` y se atasca en esa misma `y`. Aquí el "no" resulta correcto, pero el voraz no distingue un callejón real de una mala elección suya. Con las palabras `fish`, `fishy`, `yard` y el texto `fishyard`, toma `fishy`, se queda con `ard` y dice que no, cuando `fish` + `yard` funciona. La tabla nunca se compromete: conserva todos los cortes alcanzables.

## Pasos

1. Pon `ok[0]` a verdadero y el resto a falso.
2. Para cada `end` de 1 a `n`, y cada palabra en orden: si `ok[end - len(word)]` es verdadero y el texto desde ahí hasta `end` es la palabra, marca `ok[end]` y para.
3. Devuelve `[1]` si `ok[n]` y `[0]` si no.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n · palabras · L)`: cada final prueba cada palabra, comparando hasta `L` letras |
| Espacio extra | `O(n)` para `ok` |

## Errores comunes

- **Comprometerse con un solo corte.** Cortar de forma voraz, primero la más larga o la más corta, puede bloquear una palabra posterior.
- **Leer antes del principio.** Comprueba `start ≥ 0` antes de mirar `ok[start]`.
- **Esperar los trozos.** Esta tabla solo dice sí o no; para obtener un corte, guarda también qué palabra marcó cada `ok[end]`.

## Cuándo usarlo

Úsalo para segmentar texto sin espacios, comprobar si un código está hecho de partes conocidas o encajar una cadena con un conjunto de piezas.

## Temas relacionados

- [Cambio de monedas](learn:dynamic-programming/coin-change): llegar a un total exacto con piezas reutilizables.
- [Trie (árbol de prefijos)](learn:trees/trie): encontrar en un solo recorrido todas las palabras que empiezan en una posición.
- [Subir escaleras](learn:dynamic-programming/climbing-stairs): cada celda se alcanza desde celdas anteriores con saltos fijos.
- [Juego de saltos](learn:greedy/jump-game): una pregunta de alcance en la que el voraz sí es seguro.
