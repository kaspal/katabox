## La idea

Una pila es un montón en el que solo tocas lo de arriba. **push** pone un valor encima, **pop** quita el valor de arriba y lo devuelve, y **peek** lee el de arriba sin quitarlo. El último en entrar siempre es el primero en salir (LIFO), como los platos apilados en un armario.

Esa única regla es la que hace útiles las pilas. Siempre que una tarea abre algo y después tiene que cerrarlo en orden inverso (paréntesis, llamadas a funciones anidadas, carpetas en las que entras y sales, cambios que deshacer), lo próximo que hay que cerrar es siempre lo más reciente que sigue abierto: la cima de la pila.

### Sobre un arreglo dinámico

El ejemplo guarda la pila en un arreglo dinámico cuyo **final** es la cima. push añade al final, pop quita el último valor y peek lo lee; ninguno desplaza nada. El fondo de la pila es la posición 0 y nunca se mueve.

pop y peek sobre una pila vacía no tienen ningún valor que dar. Aquí devuelven `−1`.

## Pasos

1. Empieza con un arreglo `stack` vacío y una fila de salida vacía.
2. push `x`: añade `x` al final de `stack`.
3. pop: si `stack` está vacía, saca `−1`; si no, quita el último valor y sácalo.
4. peek: si `stack` está vacía, saca `−1`; si no, saca el último valor y déjalo donde está.
5. Al terminar todas las operaciones, la fila de salida tiene cada valor que devolvieron pop y peek, en orden.

## Coste

| Operación | Coste |
|---|---|
| push | `O(1)` amortizado (el arreglo puede duplicarse) |
| pop | `O(1)` |
| peek | `O(1)` |
| Espacio extra | `O(n)` para `n` valores en la pila |

## Errores comunes

- **Hacer pop sobre una pila vacía.** En Python `[].pop()` lanza un error y en JavaScript devuelve `undefined`. Comprueba si está vacía antes de cada pop y peek.
- **Usar el principio como cima.** Hacer push y pop en la posición 0 de un arreglo desplaza todos los valores, así que cada operación cuesta `O(n)`. Deja la cima al final.
- **Confundir pop con peek.** peek deja el valor en la pila; llamar a pop "solo para mirar" lo pierde.

## Cuándo usarlo

Usa una pila siempre que lo más reciente sin terminar sea lo siguiente que hay que atender: emparejar paréntesis, evaluar expresiones, el historial de deshacer, recorrer una ruta de carpetas con `..` y la búsqueda en profundidad sin recursión. Una pila monótona, que saca los valores que ya no pueden importar, responde preguntas de "siguiente valor mayor" en una sola pasada.

## Temas relacionados

- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): la pila que ya usa cada llamada a una función.
- [Pila monótona](learn:searching/monotonic-stack): una pila que mantiene sus valores en orden.
- [Coste amortizado: el arreglo dinámico](learn:foundations/dynamic-array): por qué push sigue siendo `O(1)` de media.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): la versión de primero en entrar, primero en salir.
- Búsqueda en profundidad (próximamente): explora un grafo con una pila.
