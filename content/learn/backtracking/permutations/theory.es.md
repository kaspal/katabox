## La idea

Una **permutación** es una forma de ordenar todos los valores; `n` valores tienen `n!`. La vuelta atrás construye cada orden llenando posiciones de izquierda a derecha: para la siguiente posición prueba cada valor que aún no se ha usado, sigue con la posición de después y luego retira el valor y prueba el siguiente.

El ejemplo mantiene dos piezas de estado compartido: `chosen`, el orden construido hasta ahora, y `used`, una marca por cada **posición de la entrada**. Cada llamada, `place()`, comprueba primero si `chosen` está lleno; si lo está, lo apunta y vuelve. Si no, recorre `k` de 0 a `n − 1` y, para cada posición aún sin usar, la marca, añade `values[k]`, se llama a sí misma y después quita el valor y borra la marca. Las dos piezas de estado se restauran, así que la siguiente elección parte de cero.

Las elecciones se hacen por posición, no por valor, así que el número es siempre `n!`. Con valores iguales, el mismo orden sale más de una vez: `[5, 5]` da `[5, 5]` dos veces. Cuando la entrada está ordenada, los órdenes salen en orden creciente (de diccionario). La salida es el número de permutaciones y después las permutaciones una tras otra, de `n` valores cada una. La aplicación limita `n ≤ 4`, como mucho 24 órdenes.

### La entrada por defecto

`[1, 2, 3]`. La primera llamada tiene tres opciones. Bajo el 1, las posiciones restantes dan `[1, 2, 3]` y después `[1, 3, 2]`. Bajo el 2 salen `[2, 1, 3]` y `[2, 3, 1]`; bajo el 3, `[3, 1, 2]` y `[3, 2, 1]`.

El esquema tiene 1 + 3 + 6 + 6 = 16 llamadas: una arriba, tres con un valor, seis con dos y seis llenas que se apuntan. La salida es `[6, 1, 2, 3, 1, 3, 2, 2, 1, 3, 2, 3, 1, 3, 1, 2, 3, 2, 1]`.

## Pasos

1. Empieza con `chosen` vacío y todas las marcas de `used` apagadas, y llama a `place()`.
2. Si `chosen` tiene `n` valores, apúntalo y vuelve.
3. Si no, para cada posición `k` sin usar: márcala, añade `values[k]`, llama a `place()`, quita el valor y borra la marca.
4. Devuelve el número de permutaciones y después las permutaciones apuntadas.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n · n!)`: `n!` órdenes de `n` valores cada uno, y cada llamada revisa `n` marcas |
| Espacio extra | `O(n)` para la profundidad de llamadas, `chosen` y `used`, aparte de la salida |

## Errores comunes

- **Restaurar solo la mitad del estado.** Quitar el valor pero dejar `used[k]` encendido oculta esa posición a todas las ramas posteriores.
- **Duplicados.** Para enumerar cada orden distinto una sola vez, ordena los valores y salta un valor igual al de la posición anterior mientras esa anterior esté sin usar.
- **El crecimiento.** `10!` ya son 3 628 800 órdenes. Si solo necesitas el mejor, busca una poda o una tabla.
- **Intercambiar en el sitio.** Colocar los valores con intercambios evita `used`, pero cambia el orden en que aparecen las permutaciones.

## Cuándo usarlo

Úsalo cuando el orden importa y `n` es pequeño: distribuir asientos, decidir en qué orden hacer unas pocas tareas, probar todas las rutas por un puñado de paradas. Para la ruta más barata por muchas paradas, la programación dinámica sobre subconjuntos es mucho más rápida.

## Temas relacionados

- [Subconjuntos](learn:backtracking/subsets): el mismo elegir y deshacer, cuando el orden no importa.
- [N reinas](learn:backtracking/n-queens): una permutación de columnas con comprobaciones extra.
- [Viajante de comercio (PD con máscaras)](learn:dynamic-programming/bitmask-tsp): el mejor orden sin enumerarlos todos.
- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): cómo se apilan y se deshacen las llamadas.
