## Enfoque: algoritmo de Euclides

Una baldosa cuadrada de lado `s` llena el suelo sin cortes justo cuando `s` divide a los dos lados. El mayor `s` así es el máximo común divisor de `width` y `height`.

El algoritmo de Euclides cambia una y otra vez el par `(a, b)` por `(b, a mod b)`. Todo número que divide a `a` y a `b` divide también a `a mod b = a − q·b`, y al revés, así que los divisores comunes no cambian. Cuando `b` llega a 0, `a` es la respuesta.

```
mientras height != 0:
    width, height = height, width % height
devolver width
```

### Paso a paso

`(240, 180) → (180, 60) → (60, 0)`: la respuesta es 60.

**Complejidad:** O(log min(width, height)) pasos: los números bajan al menos tan rápido como crecen los de Fibonacci. Probar todos los tamaños desde el lado menor es O(min(width, height)), unos 10¹² pasos con los suelos más grandes.
