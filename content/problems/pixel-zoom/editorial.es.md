## Enfoque: ensanchar cada fila y repetirla

Ampliar por `K` estira el dibujo en dos direcciones independientes:

- **a lo ancho**, cada carácter se convierte en `K` copias de sí mismo: `#.#` con `K = 2` pasa a `##..##`;
- **a lo alto**, cada fila ensanchada se escribe `K` veces.

Así que, para cada una de las `R` filas del dibujo, construye su fila ancha una sola vez y añádela a la salida `K` veces:

```
para cada fila del dibujo:
    ancha = la fila con cada carácter repetido K veces
    añade ancha a la salida K veces
```

**Por qué es correcto:** en el dibujo ampliado, el carácter de la línea `y`, columna `x` (desde 0) pertenece al bloque del píxel de la fila `y / K`, columna `x / K` (división entera), y debe ser una copia suya. En la fila ancha construida a partir de la fila `r`, la columna `x` contiene el carácter `x / K` de esa fila; y las líneas `r · K` a `r · K + K − 1` son exactamente las `K` copias de esa fila ancha, que son las líneas con `y / K = r`. Por tanto, cada carácter de la salida coincide con su píxel.

**Complejidad:** tiempo O(R · C · K²), que es el tamaño de la salida (como mucho 320 líneas de 320 caracteres, unos 10⁵ caracteres). No se puede hacer más rápido, porque hay que escribir cada carácter.

**Lectura y escritura:** lee toda la entrada de una vez. Las filas no tienen espacios, así que puedes dividir toda la entrada por espacios en blanco (tres números y luego `R` filas) o dividirla en líneas; con líneas, quita un `\r` final por si la entrada tiene saltos de línea de Windows. Junta las líneas de salida y escríbelas de una vez. Repetir una cadena: `ch * k` (Python), `ch.repeat(k)` (TypeScript), `str_repeat($ch, $k)` (PHP), `.repeat(k)` sobre un `&str` (Rust).
