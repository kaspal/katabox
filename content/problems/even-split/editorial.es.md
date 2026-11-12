## Enfoque: una media y una resta por amigo

La parte justa es `total / n`, donde `total` es la suma de todos los pagos. El amigo `i` pagó `paid[i]`, así que su saldo es `paid[i] − total / n`. Los saldos siempre suman cero (salvo redondeo): lo que se les debe a unos lo deben los otros.

```
parte = sum(paid) / len(paid)
devolver [p - parte para p en paid]
```

### Paso a paso

`paid = [30.0, 10.5, 20.25, 0.0]`: el total es 60.75 y la parte, 15.1875, así que los saldos son 14.8125, −4.6875, 5.0625 y −15.1875.

### Sobre la precisión

Importes como 10.1 no tienen representación binaria exacta, así que los saldos calculados pueden variar en las últimas cifras. Por eso se aceptan respuestas a menos de 10⁻⁶. Sumar 2·10⁴ importes de como mucho 10⁴ deja el error muy por debajo de eso.

**Complejidad:** O(n) de tiempo y O(n) para la respuesta.
