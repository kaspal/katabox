## La idea

Una cola es una fila: **enqueue** añade un valor al final y **dequeue** saca el valor del principio. El primero en entrar es el primero en salir (FIFO). Montarla sobre un arreglo normal tiene una pega: sacar el valor del principio obliga a desplazar todo lo demás a la izquierda, lo que cuesta `O(n)` por cada dequeue.

Un **búfer circular** evita desplazar nada. Usa un arreglo fijo de `capacity` casillas y dos números: `head`, la casilla del valor del principio, y `size`, cuántos valores hay en la cola. El final no se guarda; se calcula como `tail = (head + size) mod capacity`. Los valores nunca se mueven. dequeue solo avanza `head` una casilla, y enqueue escribe en `tail`.

### Dar la vuelta

El `mod capacity` es lo que convierte el arreglo en un anillo. Cuando el final pasa de la última casilla, sigue en la casilla 0 y reutiliza el espacio que liberaron los dequeue anteriores. Con capacidad 4, `head = 2` y `size = 2`, el siguiente valor va a `(2 + 2) mod 4 = 0`. `head` da la vuelta igual cuando avanza.

### Llena y vacía

Como la capacidad es fija, un enqueue con el búfer lleno (`size == capacity`) se rechaza. Un dequeue con el búfer vacío no tiene nada que dar y aquí devuelve `−1`.

## Pasos

1. Crea `data` con `capacity` casillas y pon `head = 0`, `size = 0`.
2. enqueue `x`: si `size == capacity`, recházalo. Si no, escribe `x` en `(head + size) mod capacity` y suma 1 a `size`.
3. dequeue: si `size == 0`, devuelve `−1`.
4. Si no, lee `data[head]`, pon `head = (head + 1) mod capacity`, resta 1 a `size` y devuelve el valor.

## Coste

| Operación | Coste |
|---|---|
| enqueue | `O(1)` |
| dequeue | `O(1)` |
| Espacio extra | `O(capacity)`, fijado de antemano |

## Errores comunes

- **Guardar `head` y `tail` sin `size`.** Cuando `head == tail`, el búfer puede estar lleno o vacío. Guardar `size` (o dejar siempre una casilla sin usar) distingue los dos casos.
- **Olvidar la vuelta.** Escribir en `head + size` sin `mod capacity` se sale del arreglo.
- **Sacar del principio de una lista.** `list.pop(0)` en Python o `shift()` en JavaScript mueven todos los demás valores; un bucle de ellos es `O(n²)`. Usa un búfer circular o una bicola.

## Cuándo usarlo

Usa una cola siempre que el trabajo deba atenderse en orden de llegada: tareas esperando a un trabajador, peticiones en un limitador de ritmo o la frontera de una búsqueda en anchura. Un búfer circular va bien con flujos de tamaño fijo: búferes de audio y de red, los últimos `k` eventos o un productor y un consumidor que se pasan datos sin reservar memoria.

## Temas relacionados

- [Pila](learn:linear/stack): la versión de último en entrar, primero en salir.
- [Bicola y cola monótona](learn:linear/deque-monotonic-queue): una cola que también trabaja por el final.
- [Arreglo dinámico: insertar y borrar](learn:linear/dynamic-array-ops): por qué sacar el principio de un arreglo es lento.
- [Ventana deslizante](learn:searching/sliding-window): una ventana que entra por un extremo y sale por el otro.
- Búsqueda en anchura (próximamente): explora un grafo nivel a nivel con una cola.
