Un mercadillo es una fila de puestos; el puesto `i` vende un tipo de producto, escrito como la letra `stalls[i]`. Tu lista de la compra `need` es una cadena de letras: cada letra es un producto que debes comprar, y una letra que aparece varias veces significa que debes visitar ese número de puestos de ese tipo (cada puesto te vende un artículo).

Quieres recorrer el menor número posible de puestos **consecutivos**. Devuelve la longitud del tramo más corto de puestos consecutivos donde puedes comprar todo lo de la lista, o `-1` si el mercadillo no tiene puestos suficientes.

### Ejemplo 1

```
stalls = "bcaacbdab", need = "aab"
Salida: 4
```

Por ejemplo los puestos "bcaa" (posiciones 0 a 3). Ningún grupo de 3 puestos consecutivos tiene dos `a` y una `b`.

### Ejemplo 2

```
stalls = "xyz", need = "xx"
Salida: -1
```

### Ejemplo 3

```
stalls = "mango", need = "m"
Salida: 1
```

### Restricciones

- 1 ≤ stalls.length, need.length ≤ 10⁵
- Las dos cadenas tienen solo letras minúsculas del alfabeto inglés.
