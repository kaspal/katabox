## La idea

Para invertir una lista simplemente enlazada en el sitio no mueves ningún valor. Das la vuelta a cada flecha: el `next` de cada nodo tiene que apuntar al nodo que antes iba **delante** de él. Lo consigues con un solo recorrido de la cabeza al final, con tres punteros:

- `cur`, el nodo que estás girando;
- `prev`, el nodo anterior, que es adonde debe apuntar ahora `cur.next` (a nada, para el primer nodo, que pasa a ser el nuevo final);
- `nxt`, el resto de la lista, guardado antes de que el giro destruya el único enlace hacia él.

Cada vuelta guarda `cur.next` en `nxt`, hace que `cur.next` apunte a `prev` y avanza: `prev` pasa a ser `cur` y `cur` pasa a ser `nxt`. Cuando `cur` se sale del final, `prev` es el antiguo último nodo, que es la nueva cabeza.

### Por qué importa el puntero guardado

En cuanto escribes `cur.next = prev`, el nodo ya no sabe dónde está el resto de la lista. Si no hubieras guardado `nxt` antes, todo lo que va detrás de `cur` quedaría inalcanzable. El orden "guardar, girar, avanzar" es todo el algoritmo.

## Pasos

1. Pon `prev = None` y `cur = head`.
2. Mientras `cur` exista, guarda `nxt = cur.next`.
3. Gira: `cur.next = prev`.
4. Avanza: `prev = cur` y luego `cur = nxt`.
5. Cuando `cur` esté vacío, devuelve `prev` como nueva cabeza.

## Coste

| Medida | Coste |
|---|---|
| Tiempo | `O(n)`: una visita por nodo, un giro cada una |
| Espacio extra | `O(1)`: tres punteros |
| Versión recursiva | `O(n)` de tiempo, pero `O(n)` de pila de llamadas |

## Errores comunes

- **Girar antes de guardar.** Si haces primero `cur.next = prev` y luego `nxt = cur.next`, obtienes `nxt = prev`: en la primera vuelta eso es nada, así que el bucle se para tras un nodo y el resto de la lista se pierde.
- **Devolver la cabeza antigua.** Tras el bucle, la cabeza antigua es el **último** nodo. La nueva cabeza es `prev`, no `head` ni `cur` (que está vacío).
- **Olvidar el enlace de la cabeza antigua.** Empezar con `prev = None` es lo que hace que la cabeza antigua apunte a nada. Si empiezas en el segundo nodo, la cabeza antigua conserva su `next` y los dos últimos nodos se apuntan entre sí: un ciclo.
- **Recursión con listas largas.** Una inversión recursiva es elegante pero usa un marco de pila por nodo; una lista de un millón de nodos puede desbordar la pila.

## Cuándo usarlo

Invertir es más una pieza que un objetivo. Aparece al comprobar si una lista es un palíndromo (invierte la segunda mitad y compara), al invertir grupos de `k` nodos, al sumar números guardados dígito a dígito y al reordenar una lista intercalando sus dos mitades. Cualquier tarea que necesite leer una lista simplemente enlazada al revés puede invertirla, leerla y volver a invertirla.

## Temas relacionados

- [Lista simplemente enlazada](learn:linear/singly-linked-list): la estructura que se reenlaza.
- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): la versión recursiva y su pila.
- [Detección de ciclos de Floyd](learn:linear/floyd-cycle): otro recorrido con punteros; con un puntero rápido también encuentra el centro.
- [Dos punteros](learn:searching/two-pointers): leer desde los dos extremos, algo que los arreglos permiten directamente.
