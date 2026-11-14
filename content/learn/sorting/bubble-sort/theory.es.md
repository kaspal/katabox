## La idea

La ordenación de burbuja recorre el arreglo comparando **vecinos**. Cada vez que un valor es mayor que el de su derecha, los dos intercambian su lugar. Tras una pasada completa, el valor más grande ha sido arrastrado hasta el final, como una burbuja que sube a la superficie, y ya no vuelve a moverse. Cada pasada siguiente puede terminar una posición antes.

## Pasos

1. Empieza una pasada sobre la parte sin ordenar, desde la primera posición hasta la última sin ordenar.
2. Compara `a[j]` con `a[j + 1]`.
3. Si `a[j]` es mayor, intercámbialos.
4. Al terminar la pasada, el último valor sin ordenar ya está en su lugar definitivo.
5. Si una pasada entera no hizo ningún intercambio, el arreglo está ordenado: para antes.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n)` (ya ordenado: una pasada, sin intercambios) |
| Caso medio | `O(n²)` |
| Peor caso | `O(n²)` (entrada invertida) |
| Espacio extra | `O(1)` |
| Estable | Sí: los valores iguales nunca se intercambian |
| En el sitio | Sí |

## Errores comunes

- **Olvidar la salida temprana.** Sin la bandera `swapped`, una entrada ordenada sigue costando `n²/2` comparaciones. Prueba la entrada "Casi ordenada" y mira qué pocas pasadas necesita.
- **Intercambiar valores iguales.** Usar `>=` en lugar de `>` hace la ordenación inestable y trabaja de más con la entrada "Pocos distintos".
- Cada intercambio corrige exactamente un par desordenado, así que el número de intercambios es el número de esos pares (inversiones). La entrada invertida es la que más tiene.

## Cuándo usarlo

Casi nunca con datos reales: la ordenación por inserción hace el mismo trabajo con menos escrituras. Merece la pena conocerla porque su número de intercambios mide lo desordenado que está un arreglo, y porque "intercambia vecinos hasta que nada cambie" es la idea de varios problemas de tipo rompecabezas.

## Temas relacionados

- [Ordenación por inserción](learn:sorting/insertion-sort): también `O(n²)`, pero desplaza en lugar de intercambiar y es más rápida en la práctica.
- [Ordenación rápida](learn:sorting/quick-sort): `O(n log n)` en promedio.
