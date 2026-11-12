Un solador cubre un suelo rectangular de `width` × `height` centímetros con baldosas **cuadradas** iguales. Las baldosas no se pueden cortar y el suelo debe quedar cubierto exactamente, de borde a borde y sin huecos.

Devuelve el lado, en centímetros enteros, de la baldosa cuadrada **más grande** que sirve.

### Ejemplo 1

```
width = 240, height = 180
Salida: 60
```

Las baldosas de 60 cm caben 4 a lo ancho y 3 a lo alto. Una de 120 cm cabría dos veces a lo ancho, pero no a lo alto.

### Ejemplo 2

```
width = 35, height = 35
Salida: 35
```

Una sola baldosa cubre todo el suelo.

### Ejemplo 3

```
width = 64, height = 27
Salida: 1
```

### Restricciones

- 1 ≤ width, height ≤ 10¹²
