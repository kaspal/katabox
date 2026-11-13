Sin obras, un paseo es elegir cuáles `rows` de sus `rows + cols` pasos van al sur: C(rows + cols, rows) paseos. Un DP sobre todos los cruces llevaría 10¹⁰ pasos, así que cuenta los paseos con fórmulas.
---
Cuenta los paseos malos por el **primer** cruce cortado que tocan. Si `good[i]` es el número de paseos desde la salida hasta el cruce cortado `i` que evitan todos los demás cruces cortados, los paseos cuyo primer cruce cortado es `i` son `good[i] · caminos(i → final)`.
---
Ordena los cruces cortados (por fila y luego columna) y añade el destino al final. `good[i] = caminos(salida → i) − Σ good[j] · caminos(j → i)` sobre los cruces anteriores `j` que pueden llegar a `i`. La respuesta es `good[destino]`. Usa tablas de factoriales e inversos modulares para `caminos`.
