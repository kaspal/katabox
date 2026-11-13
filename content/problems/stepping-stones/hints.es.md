No necesitas la ruta exacta: solo hasta dónde podría llegar el excursionista.
---
Recorre las piedras de izquierda a derecha llevando `reach`, la piedra más lejana alcanzable hasta ahora. Toda piedra hasta `reach` es alcanzable.
---
En la piedra `i`: si `i > reach`, el excursionista está atascado (false); si no, `reach = max(reach, i + jumps[i])`. Si recorres todas las piedras, la última es alcanzable.
