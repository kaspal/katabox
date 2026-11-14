## La idea

La ordenación por inserción hace crecer una parte ordenada a la izquierda, un valor cada vez, igual que mucha gente ordena las cartas que tiene en la mano. Saca el siguiente valor (la **clave**), desplaza un lugar a la derecha cada valor mayor de la parte ordenada para abrir un hueco, y coloca la clave en el hueco.

## Pasos

1. El primer valor, solo, ya es una parte ordenada.
2. Saca el siguiente valor como `key`; su lugar queda como hueco.
3. Mientras el valor a la izquierda del hueco sea mayor que la clave, desplázalo a la derecha, al hueco.
4. Coloca la clave en el hueco.
5. Repite hasta haber insertado todos los valores.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n)` (ya ordenado: sin desplazamientos) |
| Caso medio | `O(n²)` |
| Peor caso | `O(n²)` (entrada invertida) |
| Espacio extra | `O(1)` |
| Estable | Sí: la clave nunca adelanta a un valor igual |
| En el sitio | Sí |

## Errores comunes

- **Comparar con `>=`.** Mover la clave más allá de valores iguales rompe la estabilidad. El bucle debe parar en el primer valor que no sea mayor.
- **Perder la clave.** Desplazar sobrescribe `a[i]`, así que hay que guardar la clave antes de empezar el bucle.
- El trabajo es el número de pares desordenados: la entrada "Casi ordenada" sale casi gratis y la "Invertida" es el peor caso.

## Cuándo usarlo

Con arreglos pequeños (unas decenas de valores) y con datos casi ordenados, la ordenación por inserción suele ser la opción más rápida. Las bibliotecas reales la usan para los tramos cortos dentro de la ordenación por mezcla y la rápida. Además funciona **en línea**: puede ordenar los valores a medida que llegan.

## Temas relacionados

- [Ordenación de burbuja](learn:sorting/bubble-sort): la misma familia `O(n²)`, con intercambios en lugar de desplazamientos.
- [Ordenación rápida](learn:sorting/quick-sort): para arreglos grandes.
