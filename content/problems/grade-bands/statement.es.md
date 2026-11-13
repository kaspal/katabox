Una profesora convierte las notas de un examen (de 0 a 100) en franjas con letra para los boletines:

| Nota | Franja |
| --- | --- |
| de 90 a 100 | `"A"` |
| de 80 a 89 | `"B"` |
| de 70 a 79 | `"C"` |
| de 60 a 69 | `"D"` |
| menos de 60 | `"F"` |

Dadas las notas de la clase, `scores`, devuelve la franja de cada nota, en el mismo orden.

### Ejemplo 1

```
scores = [95, 82, 71, 64, 30]
Salida: ["A", "B", "C", "D", "F"]
```

### Ejemplo 2

```
scores = [90, 89, 60, 59]
Salida: ["A", "B", "D", "F"]
```

Los bordes de una franja le pertenecen: 90 es una A y 60 es una D.

### Ejemplo 3

```
scores = [100, 0]
Salida: ["A", "F"]
```

### Restricciones

- 1 ≤ scores.length ≤ 10⁴
- 0 ≤ scores[i] ≤ 100
