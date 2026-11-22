Rosa equipa un kit escolar de ciencias con poco presupuesto y necesita el coste total de los `k` artículos más baratos de la lista del proveedor. Su script usa un montículo para quedarse solo con `k` precios mientras lee la lista, pero en la mayoría de las listas los totales salen demasiado altos.

Dados `prices` y `k`, devuelve la suma de los `k` precios más pequeños. Los precios iguales son artículos distintos.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
prices = [7, 2, 9, 4, 4], k = 3
Salida: 10
```

Los tres artículos más baratos cuestan 2, 4 y 4.

### Ejemplo 2

```
prices = [19,28,26,25], k = 1
Salida: 19
```

### Ejemplo 3

```
prices = [3,3,12,27], k = 1
Salida: 3
```

### Restricciones

- 1 ≤ prices.length ≤ 10⁵
- 1 ≤ prices[i] ≤ 10⁴
- 1 ≤ k ≤ prices.length
