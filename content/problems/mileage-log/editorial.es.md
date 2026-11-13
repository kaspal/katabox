## Enfoque: sumas prefijas

Calcula los totales acumulados una sola vez: `prefix[0] = 0` y `prefix[i + 1] = prefix[i] + miles[i]`. Entonces el total de los días `a..b` es el total hasta `b` menos el total anterior a `a`:

```
viaje(a, b) = prefix[b + 1] − prefix[a]
```

### Paso a paso

`miles = [4, 0, 7, 3, 5]` da `prefix = [0, 4, 4, 11, 14, 19]`. El viaje [1, 3] es `prefix[4] − prefix[1] = 14 − 4 = 10`.

**Complejidad:** O(n + q) en tiempo y O(n) de espacio extra. Sumar cada viaje directamente es O(n·q), hasta 10¹⁰ sumas.
