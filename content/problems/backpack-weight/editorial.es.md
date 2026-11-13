## Enfoque: mochila 0/1 sobre las capacidades

Sea `best[c]` = el mayor valor que cabe en la capacidad `c` usando los objetos procesados hasta ahora; empieza con todo a cero.

Para cada objeto de peso `w` y valor `v`, actualiza las capacidades de mayor a menor:

```
para c = capacity hasta w, bajando:
    best[c] = max(best[c], best[c − w] + v)
```

`best[c − w]` aún describe los objetos **anteriores** a este (todavía no se ha actualizado en esta pasada, porque vamos hacia abajo), así que el objeto cuenta como mucho una vez. La respuesta es `best[capacity]`.

### Paso a paso

Ejemplo 1: tras los objetos 0 (3, 30) y 1 (4, 50), `best[7] = 80`. El objeto 2 (2, 15) da `best[7] = max(80, best[5] + 15) = max(80, 65)`. El objeto 3 (5, 60) da `max(80, best[2] + 60) = max(80, 75)`. La respuesta sigue siendo 80.

**Complejidad:** O(n · capacity) en tiempo y O(capacity) de espacio: como mucho 2·10⁶ pasos. Probar todos los subconjuntos es O(2ⁿ).
