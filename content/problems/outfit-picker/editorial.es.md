## Enfoque: backtracking, una categoría cada vez

Construye el conjunto categoría a categoría con una función recursiva. `elegir(c, gastado)` devuelve de cuántas formas se pueden completar las categorías `c, c + 1, …` sin pasarse del presupuesto, sabiendo que ya se ha gastado `gastado`:

```
elegir(c, gastado):
    si c == número de categorías: devolver 1          # un conjunto completo
    total = 0
    para precio en los precios ordenados de la categoría c:
        si gastado + precio + mínimo(c + 1 ..) > budget: cortar
        total += elegir(c + 1, gastado + precio)
    devolver total
respuesta = elegir(0, 0)
```

`mínimo(c + 1 ..)` es la suma de la prenda más barata de cada categoría posterior: si ni siquiera esas caben, ninguna forma de completar el conjunto cabe. Como cada categoría está ordenada, en cuanto una prenda es demasiado cara también lo son las siguientes, y el bucle se corta. Esta **poda** es lo que hace al backtracking más rápido que listar todos los conjuntos, aunque con 6⁶ = 46 656 conjuntos como mucho incluso la lista completa es pequeña.

### Paso a paso

`prices = [[30, 50], [20, 40, 60], [10]]`, presupuesto 100. Con el sombrero de 30: las chaquetas de 20, 40 y 60 caben (con los zapatos de 10: 60, 80, 100), 3 conjuntos. Con el de 50: caben 20 y 40 (80, 100), pero 50 + 60 + 10 = 120 no, y el bucle se corta: 2 conjuntos. Total 5.

**Complejidad:** O(conjuntos que caben) llamadas recursivas en el peor caso, 6⁶ como mucho, con una profundidad igual al número de categorías.
