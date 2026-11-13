## Enfoque: comprobar cada año

Recorre los años de `start` a `end` y cuenta los que cumplen

```
y % 4 == 0 and (y % 100 != 0 or y % 400 == 0)
```

Los paréntesis importan: la excepción de los múltiplos de 100 solo se aplica a años divisibles entre 4, y la excepción a la excepción (400) solo a los múltiplos de 100.

**Complejidad:** O(end − start) en tiempo, como mucho 9999 comprobaciones.

**Para ir más allá:** el número de bisiestos hasta `y` es `y/4 − y/100 + y/400` (división entera), así que la respuesta es `L(end) − L(start − 1)` en O(1).
