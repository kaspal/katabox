## Enfoque: prefijo palíndromo más largo con la función de prefijos

**Cómo es la respuesta.** Supón que añadimos `k` letras `x` delante, de modo que el resultado es `p = x + s`, con `n = |s|`. Si `p` es un palíndromo, sus `k` primeras letras reflejan sus `k` últimas, así que `x` son las `k` últimas letras de `s`, al revés. Lo que queda entre ellas, las `n − k` primeras letras de `s`, tiene que reflejarse a sí mismo: es un palíndromo. Al revés, si `s[0 : n − k]` es un palíndromo, entonces `reverse(s[n − k:]) + s` también lo es. Así que añadir el mínimo de letras equivale a encontrar el **prefijo palíndromo más largo** de `s`, de longitud `keep`, y la respuesta es

```
reverse(s[keep:]) + s
```

Para `abac` los prefijos palíndromos son `a` y `aba`, así que `keep = 3` y la respuesta es `c` + `abac` = `cabac`.

**Cómo encontrar el prefijo palíndromo más largo.** Un prefijo de longitud `L` es un palíndromo exactamente cuando coincide con su inverso, y su inverso es el sufijo de longitud `L` de `reverse(s)`. Así que buscamos el prefijo más largo de `s` que también es sufijo de `reverse(s)`. Eso es lo que mide la función de prefijos de KMP: para una cadena `t`, `pi[i]` es la longitud del prefijo propio más largo de `t[0..i]` que también es sufijo suyo. Construye

```
t = s + "#" + reverse(s)
```

y calcula `pi` en una pasada. En la última posición, `pi` es el prefijo más largo de `t` que termina la cadena `t` entera. Como `#` aparece una sola vez y nunca en `s`, esa coincidencia no puede cruzar el centro, así que mide como mucho `n`: es un prefijo de `s` que es sufijo de `reverse(s)`. Esa longitud es `keep`.

Para `abac`, `t = abac#caba` y su función de prefijos termina en `3` (`aba`).

**Por qué es correcto:** el primer párrafo muestra que todo palíndromo que termina con `s` añade al menos `n − keep` letras y que `n − keep` letras bastan; la función de prefijos encuentra `keep` exactamente. La respuesta es única, porque las `k` letras añadidas están determinadas.

**Complejidad:** tiempo y memoria O(n). El `while` interno de la función de prefijos solo retrocede lo que los pasos anteriores avanzaron, así que la pasada completa es lineal.

**El enfoque evidente es demasiado lento.** Probar los prefijos del más largo al más corto, letra a letra desde los dos extremos, es O(n²) en el peor caso. Una palabra como `a…a b a…a`, con el doble de `a` después de la `b`, hace que muchos prefijos sean casi palíndromos: con 2·10⁵ letras son unas 4.4·10⁹ comparaciones de letras.

**Notas prácticas:** lee toda la entrada de una vez y quita el salto de línea (`trim`, `split`); si no, un `\r` suelto acabaría dentro de la palabra. Escribe la respuesta, de hasta 4·10⁵ − 1 letras, de una sola vez. En TypeScript compara con `charCodeAt` y guarda `pi` en un `Int32Array`; en Rust trabaja con los bytes (`as_bytes()`), ya que todas las letras son ASCII.
