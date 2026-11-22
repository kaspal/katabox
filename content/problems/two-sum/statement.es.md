El libro de cuentas de una tienda tiene un importe por línea en `nums`: las ventas son positivas y las devoluciones negativas. La auditora sabe que exactamente dos líneas suman juntas `target` y quiere encontrarlas.

Devuelve las posiciones (desde 0) de esas dos líneas, en cualquier orden. Cada línea se puede usar una sola vez, aunque su importe sea justo la mitad de `target`.

### Ejemplo 1

```
nums = [4, 11, 7, 15, 2], target = 13
Salida: [1, 4]
```

Las líneas 1 y 4 tienen 11 y 2, que suman 13. Ningún otro par lo hace.

### Ejemplo 2

```
nums = [-6, 14, 3, 20, -1], target = 14
Salida: [0, 3]
```

Una devolución de 6 y una venta de 20 suman 14. La línea de 14 no puede emparejarse consigo misma.

### Restricciones

- 2 ≤ nums.length ≤ 10⁵
- −10⁹ ≤ nums[i], target ≤ 10⁹
- Exactamente un par de líneas suma `target`.
