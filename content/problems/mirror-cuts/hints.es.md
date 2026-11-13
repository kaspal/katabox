Sea `cuts[i]` la respuesta para las `i` primeras letras. La última pieza de la mejor división es algún palíndromo `text[j..i)`.
---
`cuts[i] = mín sobre los palíndromos text[j..i) de (cuts[j] + 1)`, con `cuts[0] = −1` para que un palíndromo entero necesite 0 cortes. Necesitas saber rápido qué subcadenas son palíndromos.
---
Expande alrededor de cada centro (cada letra y cada hueco): mientras `text[l] == text[r]`, la pieza `text[l..r]` es un palíndromo, así que actualiza `cuts[r + 1] = min(cuts[r + 1], cuts[l] + 1)` y ensancha. Procesa los centros de izquierda a derecha.
