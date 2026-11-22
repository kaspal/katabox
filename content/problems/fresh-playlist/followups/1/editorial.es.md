## Idea

El problema base permitía cada canción una vez; aquí dos, y la misma ventana deslizante sigue funcionando porque acortar un tramo válido lo mantiene válido. Mueve `end` por las emisiones y cuenta cada canción dentro de `[start, end]` en un mapa hash. Solo la canción recién añadida puede haber llegado a 3 emisiones, así que, mientras su cuenta pase de 2, quita `plays[start]` y avanza `start`. La ventana es entonces el tramo válido más largo que acaba en `end`, y la respuesta es la mayor de esas longitudes. El mismo código sirve para cualquier límite de repeticiones.

## Complejidad

O(n) en tiempo, porque cada posición entra y sale de la ventana una vez. O(n) de memoria extra para las cuentas en el peor caso.
