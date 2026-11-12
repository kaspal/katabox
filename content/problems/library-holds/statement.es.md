Una biblioteca de barrio mantiene una lista de espera para cada libro prestado. Procesa una lista de sucesos, donde `ops[i]` es el nombre del suceso y `args[i]` sus números:

- `"hold"`, args `[book, member]`: el socio se pone al **final** de la lista del libro.
- `"cancel"`, args `[book, member]`: el socio sale de la lista del libro. Si no estaba en ella, no pasa nada.
- `"return"`, args `[book]`: vuelve un ejemplar del libro. Se lo lleva el socio que está **primero** en la lista, que sale de ella. **Informa** de ese socio, o de `-1` si no espera nadie (el ejemplar vuelve a la estantería).

Un socio puede esperar varios libros a la vez, y puede volver a apuntarse a una lista después de salir de ella (va al final). Un socio nunca reserva un libro cuya lista ya ocupa.

Devuelve los informes de los sucesos `"return"`, en orden.

### Ejemplo 1

```
ops = ["hold", "hold", "hold", "return", "cancel", "return", "return"], args = [[1, 10], [1, 20], [1, 30], [1], [1, 30], [1], [1]]
Salida: [10, 20, -1]
```

La lista del libro 1 es 10, 20, 30. El primer ejemplar es para el 10; el socio 30 cancela; el siguiente ejemplar es para el 20; después no espera nadie.

### Ejemplo 2

```
ops = ["return"], args = [[5]]
Salida: [-1]
```

### Ejemplo 3

```
ops = ["hold", "hold", "cancel", "hold", "return", "return"], args = [[2, 7], [2, 8], [2, 7], [2, 7], [2], [2]]
Salida: [8, 7]
```

El socio 7 cancela y vuelve a apuntarse, así que ahora está detrás del socio 8.

### Restricciones

- 1 ≤ ops.length ≤ 10⁵, args.length = ops.length
- Cada `ops[i]` es `"hold"`, `"cancel"` o `"return"`, con los números descritos arriba
- 1 ≤ book, member ≤ 10⁹
