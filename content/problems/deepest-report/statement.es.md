Rosa trabaja en recursos humanos en una empresa que crece muy deprisa y quería saber cuántos niveles separan a la directora general del empleado de nivel más bajo. Su función responde al instante para todos los departamentos que probó, pero cuando le pasó el organigrama completo de la empresa falló con un error en lugar de responder.

Hay `manager.length` personas numeradas desde 0. `manager[i]` es la persona a la que reporta `i`, y exactamente una persona, la jefa, tiene `manager[i] = -1`. La cadena de responsables de cualquier persona acaba llegando a la jefa. La jefa está en el nivel 1, y quien reporta a una persona del nivel `d` está en el nivel `d + 1`. Devuelve el nivel más profundo de la empresa.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
manager = [2, 2, -1, 0, 3, 1]
Salida: 4
```

La persona 2 es la jefa (nivel 1), 0 y 1 reportan a 2 (nivel 2), 3 y 5 están en el nivel 3, y 4 reporta a 3, así que está en el nivel 4.

### Ejemplo 2

```
manager = [3,3,-1,2]
Salida: 3
```

### Ejemplo 3

```
manager = [1,2,-1,1]
Salida: 3
```

### Restricciones

- 1 ≤ manager.length ≤ 5·10⁴
- −1 ≤ manager[i] < manager.length
- Exactamente una persona (la jefa) tiene manager −1
- Toda cadena de responsables llega a la jefa (no hay ciclos)
