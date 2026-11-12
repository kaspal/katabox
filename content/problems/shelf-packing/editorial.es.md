## Enfoque: backtracking, los más gruesos primero, con poda

Es el **empaquetado en contenedores** (bin packing), que no tiene un algoritmo exacto rápido conocido, pero 15 libros son pocos para buscar con cuidado.

1. Ordena los libros del más grueso al más fino.
2. Empieza `mejor` con una buena respuesta: **primero que quepa, decreciente** (cada libro en la primera balda donde cabe, si no, una balda nueva).
3. Busca recursivamente: `colocar(i, cargas)` pone el libro `i` en cada balda usada donde cabe, o en una balda nueva, y después coloca el libro `i + 1`. Cuando todos los libros están colocados, `mejor = min(mejor, baldas usadas)`.

La poda la hace rápida:

- **Cota:** una rama que ya usa `mejor − 1` baldas no puede mejorar abriendo una nueva, y una rama que usa `mejor` se descarta.
- **Simetría:** las baldas con la misma carga son intercambiables, así que solo se prueba la primera de ellas para un libro.
- **Cota inferior:** ninguna respuesta puede ser menor que ⌈total / width⌉; en cuanto `mejor` la alcanza, la búsqueda para.

Colocar primero los libros más gruesos hace que las ramas sobrecargadas fallen cerca de la cima del árbol, donde cortar ahorra más.

### Paso a paso

`[4, 4, 4, 3, 3, 3, 2, 2, 2]`, ancho 10, total 27, así que la cota inferior es 3. Primero que quepa decreciente da {4, 4}, {4, 3, 3}, {3, 2, 2}, {2}: 4 baldas. La búsqueda encuentra después {4, 4, 2}, {4, 3, 3}, {3, 2, 2}, que alcanza la cota inferior, y para.

**Complejidad:** exponencial en el peor caso (como mucho 15! órdenes, muchos menos con la poda), con profundidad de recursión n.
