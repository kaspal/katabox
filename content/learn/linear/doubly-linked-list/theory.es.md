## La idea

En una lista doblemente enlazada cada nodo tiene dos punteros: `next`, el nodo de detrás, y `prev`, el nodo de delante. La lista guarda los dos extremos, `head` y `tail`. Con enlaces en las dos direcciones, puedes añadir o quitar un nodo por cualquier extremo sin recorrer nada, así que la lista funciona como una **bicola**: meter y sacar por el principio y por el final, cada cosa en `O(1)`.

La pega es la contabilidad. Cada cambio toca hasta cuatro punteros, y los extremos piden cuidado: cuando la lista tiene un solo nodo, ese nodo es a la vez `head` y `tail`, y al quitarlo hay que vaciar los dos.

En una lista simplemente enlazada, pop-back es la lenta: encontrar el nodo anterior al último obliga a recorrer desde la cabeza. Aquí `tail.prev` lo da directamente.

En este tema, sacar de una lista vacía devuelve `−1`. Al final, la salida lee la lista hacia delante desde `head` y luego hacia atrás desde `tail`, lo que comprueba que las dos direcciones de enlaces coinciden.

## Pasos

1. front `x`: enlaza el nodo nuevo antes de `head` en los dos sentidos; si la lista estaba vacía, también es `tail`. Pon `head` en él.
2. back `x`: enlaza el nodo nuevo después de `tail` en los dos sentidos; si la lista estaba vacía, también es `head`. Pon `tail` en él.
3. pop-front: si está vacía, devuelve `−1`. Si no, pon `head = head.next`, vacía el `prev` de la nueva cabeza (o `tail`, si la lista se ha quedado vacía) y devuelve el valor antiguo.
4. pop-back: lo mismo con `tail`, intercambiando `prev` y `next`.

## Coste

| Operación | Coste |
|---|---|
| Meter por cualquier extremo | `O(1)` |
| Sacar por cualquier extremo | `O(1)` |
| Quitar un nodo que ya tienes | `O(1)` |
| Buscar por valor o posición | `O(n)` |
| Espacio extra | Dos punteros por nodo |

## Errores comunes

- **Actualizar una sola dirección.** Poner `next` pero no `prev` (o al revés) hace que la lectura hacia delante y la de hacia atrás no coincidan.
- **El caso de un solo nodo.** Al sacar el último nodo hay que vaciar `head` y `tail`; si olvidas uno, queda un puntero a un nodo eliminado.
- **El orden de las actualizaciones.** Guarda el nodo que quitas antes de mover `head` o `tail`, o perderás su valor.

## Cuándo usarlo

Usa una lista doblemente enlazada cuando necesites quitar o mover nodos del medio en `O(1)` teniendo un puntero a ellos: la lista de uso reciente de una caché LRU, deshacer y rehacer en un editor o una lista de reproducción que recorres en los dos sentidos. Junto a un mapa hash que apunta a los nodos, da búsquedas en `O(1)` y reordenaciones en `O(1)`.

## Temas relacionados

- [Lista simplemente enlazada](learn:linear/singly-linked-list): un puntero por nodo; insertar al final es `O(n)` sin puntero `tail`.
- [Caché LRU](learn:linear/lru-cache): un mapa hash más una lista doblemente enlazada.
- [Bicola y cola monótona](learn:linear/deque-monotonic-queue): para qué sirve una bicola.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): una alternativa sobre arreglo, sin punteros.
