## Idea

Simula la regla con dos montículos. Ordena los índices por `(start, índice)` para que los
empates sigan el orden de la entrada. Guarda `busy`, un montículo de mínimos de
`(end, sala)`, y `free`, un montículo de mínimos de números de sala. Para cada reunión,
saca primero toda sala ocupada cuyo fin no supere el inicio de la reunión y mete su número
en `free`; una sala que se libera justo en el inicio cuenta. Después asigna a la reunión
la cima de `free`, o una sala nueva con el número siguiente a las abiertas hasta ahora si
`free` está vacío, guárdala en el índice original de la reunión y mete `(end, sala)` en
`busy`. El mayor número asignado coincide con la respuesta del problema base.

## Complejidad

O(n log n) en tiempo: la ordenación, más como mucho una inserción y una extracción por
reunión en cada montículo. O(n) de memoria extra para el orden, los montículos y la
salida.
