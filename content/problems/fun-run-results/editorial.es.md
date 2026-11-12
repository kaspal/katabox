## Enfoque: ordenar por una clave de dos partes

Junta cada nombre con su tiempo y ordena las parejas por la clave `(tiempo, nombre)`. Las tuplas se comparan parte a parte: deciden los tiempos, y los nombres solo se comparan cuando los tiempos son iguales. La comparación de cadenas es alfabética, y una cadena que es el comienzo de otra más larga es menor, como pide el tablón.

```
orden = ordenar índices i por (times[i], names[i])
devolver [names[i] para i en orden]
```

Ordenar los índices (no solo los nombres) mantiene cada nombre junto a su tiempo.

### Paso a paso

Las parejas son (1810, ana), (1795, leo), (1810, bea), (2000, zoe). Ordenadas: (1795, leo), (1810, ana), (1810, bea), (2000, zoe), así que el tablón dice leo, ana, bea, zoe.

**Complejidad:** O(n log n) comparaciones (cada una de 10 letras como mucho) y O(n) de espacio extra.
