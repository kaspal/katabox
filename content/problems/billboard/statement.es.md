La valla de una feria está hecha de paneles colocados uno junto a otro, cada uno de 1 metro de ancho; `panels[i]` es la altura del panel `i`, medida desde el suelo. Un anunciante quiere pegar un cartel rectangular en la valla: su borde inferior está en el suelo, cubre paneles enteros y no puede sobresalir por encima de ningún panel que cubra.

Devuelve el área del mayor cartel posible.

### Ejemplo 1

```
panels = [3, 1, 4, 4, 2, 5]
Salida: 8
```

O bien 2 paneles de altura 4 (paneles 2-3), o bien 4 paneles de altura 2 (paneles 2-5).

### Ejemplo 2

```
panels = [6]
Salida: 6
```

### Ejemplo 3

```
panels = [2, 2, 2, 0, 3]
Salida: 6
```

### Restricciones

- 1 ≤ panels.length ≤ 10⁵
- 0 ≤ panels[i] ≤ 10⁴
