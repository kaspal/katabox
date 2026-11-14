## La idea

Una lista simplemente enlazada guarda cada valor en su propio **nodo**, y cada nodo tiene un puntero: `next`, el nodo que va detrás. La lista en sí es solo `head`, un puntero al primer nodo; el `next` del último está vacío. Los nodos pueden estar en cualquier parte de la memoria, así que añadir o quitar uno nunca desplaza a los demás. Solo cambias punteros.

El precio es que solo puedes avanzar, de nodo en nodo. No hay `a[i]`: llegar al quinto nodo exige seguir cuatro enlaces.

- **Insertar al principio** es instantáneo: el nodo nuevo apunta a la cabeza actual y luego `head` apunta al nodo nuevo.
- **Insertar al final** obliga a recorrer toda la lista para encontrar el último nodo, porque esta versión no guarda un puntero `tail`. Ese recorrido es `O(n)`.
- **Borrar por valor** recorre con dos punteros: `cur` mira cada nodo y `prev` va un nodo por detrás. Cuando `cur` tiene el valor, `prev.next` se lo salta. Si el que coincide es la propia cabeza, no hay `prev`, así que es `head` quien avanza.

## Pasos

1. front `x`: crea un nodo para `x`, pon su `next` en `head` y luego pon `head` en él.
2. back `x`: si la lista está vacía, el nodo pasa a ser `head`. Si no, avanza `cur` desde `head` hasta que `cur.next` esté vacío y luego pon `cur.next` en el nodo.
3. delete `x`: pon `prev` a nada y `cur` en `head`. Mientras `cur` exista y su valor no sea `x`, mueve los dos un nodo.
4. Si `cur` se salió del final, `x` no está en la lista: no hagas nada.
5. Si `prev` está vacío, pon `head = cur.next`; si no, pon `prev.next = cur.next`.

## Coste

| Operación | Coste |
|---|---|
| Insertar al principio | `O(1)` |
| Insertar al final (sin puntero `tail`) | `O(n)`: recorrer hasta el último nodo |
| Borrar o buscar por valor | `O(n)` |
| Espacio extra | Un puntero `next` por nodo |

## Errores comunes

- **Enlazar en el orden equivocado.** Al principio, pon `node.next = head` antes de `head = node`; al revés, pierdes la lista antigua.
- **Borrar la cabeza.** No tiene `prev`, así que necesita su propio caso. Un nodo "ficticio" colocado antes de la cabeza elimina ese caso especial.
- **Salirse del final.** Comprueba que `cur` existe antes de leer `cur.value` o `cur.next`.

## Cuándo usarlo

Las listas enlazadas brillan cuando ya tienes un puntero al sitio donde cambian las cosas: empalmar listas, quitar un nodo que ya tienes a mano o montar una pila con operaciones al principio en `O(1)` y sin redimensionar. Para leer por posición o recorrer muchos valores, un arreglo dinámico es más rápido en la práctica.

## Temas relacionados

- [Lista doblemente enlazada](learn:linear/doubly-linked-list): añade un puntero hacia atrás, así que los dos extremos son `O(1)`.
- [Invertir una lista](learn:linear/reverse-list): cambia cada `next` en una sola pasada.
- [Detección de ciclos de Floyd](learn:linear/floyd-cycle): dos punteros que avanzan a distinta velocidad.
- [Arreglo dinámico: insertar y borrar](learn:linear/dynamic-array-ops): el lado del arreglo en el mismo compromiso.
