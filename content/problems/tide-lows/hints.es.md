Recorrer cada tramo cuesta hasta `N` pasos por pregunta, y los tramos pueden ser largos. El registro no cambia nunca: ¿qué podrías calcular una sola vez, antes de las preguntas, para que cada respuesta sea rápida?
---
El nivel más bajo de un tramo se puede sacar de dos trozos que lo cubran, **aunque se solapen**: una lectura contada dos veces no cambia un mínimo. Precalcula el mínimo de cada trozo cuya longitud sea una potencia de dos.
---
Tabla dispersa (sparse table): `low[0] = niveles`, y `low[k][i] = min(low[k−1][i], low[k−1][i + 2^(k−1)])` es el mínimo de las `2^k` lecturas que empiezan en `i`. Para una pregunta, sea `k` la mayor potencia con `2^k ≤ r − l + 1`; la respuesta es `min(low[k][l], low[k][r − 2^k + 1])` (posiciones desde 0).
