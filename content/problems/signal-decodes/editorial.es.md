## Enfoque: contar las lecturas de cada prefijo

Una búsqueda recursiva que en cada posición prueba «tomar un dígito» y «tomar dos dígitos» es correcta, pero visita las lecturas una a una, y un mensaje de `n` unos tiene `Fib(n+1)` lecturas: demasiadas para 10⁵ dígitos.

Mejor contar. Sea `formas[i]` el número de lecturas de los primeros `i` dígitos `d1 … di`, con `formas[0] = 1` (el prefijo vacío tiene una lectura: ninguna letra). Toda lectura de `d1 … di` termina exactamente en una de estas dos formas:

- la letra `di` sola: posible si `di ≠ 0`, y el resto es cualquier lectura de los primeros `i-1` dígitos;
- la letra `d(i-1) di`: posible si esos dos dígitos forman un número del 10 al 26 (es decir, `d(i-1)` es `1`, o es `2` con `di ≤ 6`), y el resto es cualquier lectura de los primeros `i-2` dígitos.

Los dos casos nunca se solapan (la última letra tiene distinta longitud), así que

```
formas[i] = (di ≠ 0 ? formas[i-1] : 0) + (10 ≤ d(i-1)di ≤ 26 ? formas[i-2] : 0)
```

Los ceros se resuelven solos: un `0` solo aporta a través de la regla del par, así que `30`, `00` o un `0` inicial dejan la cuenta en 0 a partir de ahí. La respuesta es `formas[n]`, tomada módulo 1 000 003 en cada paso (la suma de dos restos queda muy por debajo de 2⁵³). Solo hacen falta los dos últimos valores, así que dos variables sustituyen al arreglo.

**Lectura de la entrada:** divide la entrada por espacios; cada token de mensaje empieza por `#`, así que lee sus dígitos desde el segundo carácter. (El `#` también evita que un mensaje de 10⁵ dígitos parezca un número).

**Complejidad:** tiempo O(dígitos en total), memoria extra O(1) por mensaje.

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` con `charCodeAt` (TypeScript), `stream_get_contents(STDIN)` con `preg_split` (PHP), `read_to_string` con `split_ascii_whitespace` y `as_bytes` (Rust).
