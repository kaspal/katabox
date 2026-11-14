Desempaquetar es imposible: una sola etiqueta puede medir miles de millones de letras. Y recorrer una etiqueta larga tramo a tramo en cada revisión sigue siendo demasiado lento cuando 20 000 revisiones caen en una etiqueta de 60 000 tramos.
---
Analiza cada etiqueta una sola vez y sepárala en tramos. La posición `k` cae en el primer tramo cuyo **final** (la suma de las cantidades hasta ese tramo, incluido) es al menos `k`. Esos finales solo crecen a lo largo de la etiqueta.
---
Guarda, por etiqueta, la lista de finales de tramo (sumas prefijas de las cantidades) y las letras de los tramos. En cada revisión, busca con búsqueda binaria el primer final ≥ `k` y escribe la letra de ese tramo. Los finales quedan por debajo de 2⁵³, así que son exactos incluso en TypeScript.
