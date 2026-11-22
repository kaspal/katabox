## El error

La única línea de la función usa el operador equivocado:

```python
return sum(grades) // len(grades)
```

`//` es la división entera: redondea el cociente hacia abajo a un número entero. Eso solo acierta cuando la suma es múltiplo de la cantidad, como en el Ejemplo 3 (88 / 4 = 22). Con `grades = [1, 2]` el código devuelve `1` en lugar de `1.5`, y en el Ejemplo 1 devuelve `82` en vez de `82.333...`.

## La corrección

Usa la división real, que conserva la parte decimal:

```python
def average_grade(grades: list[int]) -> float:
    return sum(grades) / len(grades)
```

En TypeScript, devuelve `total / grades.length` sin la llamada a `Math.floor`.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra.
