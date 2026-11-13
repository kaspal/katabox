## Enfoque: DP de cortes con palíndromos hallados al expandir centros

Sea `cuts[i]` = el menor número de cortes para el prefijo de longitud `i`, con `cuts[0] = −1` (para que un prefijo que ya es palíndromo obtenga `−1 + 1 = 0`) y `cuts[i] = i − 1` al principio (cortar entre cada par de letras).

Todo palíndromo tiene un centro: una letra (longitud impar) o un hueco (longitud par). Expandir desde un centro mientras `text[l] == text[r]` recorre todos los palíndromos `text[l..r]` a su alrededor; cada uno puede cerrar una división de `text[0..r]`:

```
cuts[r + 1] = min(cuts[r + 1], cuts[l] + 1)
```

Procesa los centros de izquierda a derecha. Cuando el centro está en la posición c, todo palíndromo que empieza en l termina en c o después, así que `cuts[l]` (con l ≤ c) solo depende de palíndromos que terminan antes de c, que ya se han visto. La respuesta es `cuts[n]`.

**Complejidad:** O(n²) en tiempo (como mucho unas 10⁶ expansiones aquí) y O(n) de espacio. Probar recursivamente todas las divisiones sin recordar resultados es exponencial.
