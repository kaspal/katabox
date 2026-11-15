La esfera solo tiene 12 horas: las 15:00 y las 03:00 se ven igual. Mide hacia dónde apunta cada aguja, como un ángulo desde el 12, y compáralas.
---
Desde el 12, el minutero está a `6 · mm` grados y la aguja horaria a `30 · (hh mod 12) + 0.5 · mm` grados. Su diferencia puede pasar de 180: entonces el otro lado de la esfera es más corto, `360 − diferencia`.
---
Cuenta en medios grados para que todo sea entero: la aguja horaria está en `60 · (hh mod 12) + mm` y el minutero en `12 · mm`, así que `gap = |60 · (hh mod 12) − 11 · mm|` y la respuesta es `min(gap, 720 − gap)` medios grados. Escribe `respuesta / 2` (división entera) y después `.5` si `respuesta` es impar y `.0` si es par.
