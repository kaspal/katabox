La profesora Okafor escribió una pequeña función para calcular la media de la clase en los boletines. Algunos alumnos notaron que sus medias salían sospechosamente redondas, y a unos cuantos les faltaba una fracción de punto.

Dado un arreglo `grades` de notas enteras, devuelve su media: la suma de todas las notas dividida entre cuántas hay. El resultado es un número decimal; se aceptan respuestas a menos de 10⁻⁶ del valor exacto.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
grades = [70, 85, 92]
Salida: 82.33333333333333
```

(70 + 85 + 92) / 3 = 247 / 3.

### Ejemplo 2

```
grades = [72, 97, 8, 32]
Salida: 52.25
```

### Ejemplo 3

```
grades = [11, 10, 46, 21]
Salida: 22.0
```

### Restricciones

- 1 ≤ grades.length ≤ 10⁵
- 0 ≤ grades[i] ≤ 100
