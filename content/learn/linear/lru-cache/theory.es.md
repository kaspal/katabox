## La idea

Una caché guarda unas pocas respuestas a mano para no tener que traerlas o calcularlas otra vez. Cuando está llena y llega una entrada nueva, algo tiene que salir. Una caché **LRU** (menos usada recientemente) expulsa la entrada que lleva más tiempo sin usarse, apostando a que lo que se usó hace poco se volverá a usar pronto.

Necesita dos cosas a la vez:

- un **mapa** de clave a valor, para que `get` encuentre una entrada al instante;
- un **orden de uso**, de la menos reciente a la más reciente, para que la caché sepa qué expulsar.

Cada `get` que encuentra su clave (un **acierto**) mueve esa clave al extremo más reciente. Un `get` que falla devuelve `−1` y no cambia nada. Un `put` escribe el valor en el extremo más reciente (moviendo antes la clave allí si ya estaba en la caché) y, si la caché tiene ahora más de `capacity` claves, expulsa la del extremo menos reciente.

### Construir el orden

Los ejemplos se apoyan en sus lenguajes: el `OrderedDict` de Python mantiene las claves en orden y puede mover una al final (`move_to_end`) o quitar la primera (`popitem(last=False)`), y un `Map` de JavaScript mantiene el orden de inserción, así que borrar una clave y volver a ponerla la mueve al final. La animación dibuja ese orden como una lista doblemente enlazada, con la menos reciente a la izquierda y el mapa al lado.

Ese dibujo es cómo se construye a mano: un mapa hash de clave a **nodo** y una lista doblemente enlazada de nodos. El mapa encuentra el nodo en `O(1)` y, con los enlaces `prev` y `next`, el nodo se puede desenganchar y llevar al final en `O(1)`.

## Pasos

1. get `k`: si `k` no está en el mapa, devuelve `−1` (un fallo).
2. Si está, mueve `k` al extremo más reciente y devuelve su valor (un acierto).
3. put `k v`: si `k` está en la caché, muévela al extremo más reciente. Guarda `v` para `k` en ese extremo.
4. Si ahora hay más de `capacity` claves, expulsa la del extremo menos reciente.

## Coste

| Operación | Coste |
|---|---|
| get | `O(1)` |
| put | `O(1)`, expulsión incluida |
| Espacio extra | `O(capacity)`: una entrada del mapa y un nodo de la lista por clave |

## Errores comunes

- **Olvidar que `get` cambia el orden.** Una lectura cuenta como uso. Si no mueves la clave, la caché se convierte en primero en entrar, primero en salir.
- **Expulsar antes de escribir.** Actualizar una clave que ya está en la caché nunca necesita expulsar; comprueba el tamaño después de escribir, no antes.
- **Quitar de un solo lado.** Al expulsar, borra la clave del mapa y el nodo de la lista.

## Cuándo usarlo

Usa una caché LRU cuando las peticiones recientes predicen las siguientes: páginas web, filas de una base de datos o resultados de llamadas lentas a funciones (`functools.lru_cache` de Python es justo esto).

## Temas relacionados

- [Lista doblemente enlazada](learn:linear/doubly-linked-list): el orden de uso con movimientos en `O(1)`.
- [Lista simplemente enlazada](learn:linear/singly-linked-list): por qué aquí no basta con un enlace por nodo.
- Tablas hash (próximamente): la mitad de mapa de la caché.
