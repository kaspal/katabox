## Enfoque: descifrar el bloque de texto en una pasada

**Lectura.** La primera línea es la clave; todo lo que sigue es texto de longitud desconocida, y su forma exacta importa: los espacios iniciales, los signos y las líneas vacías tienen que salir sin cambios. Así que no lo dividas en tokens. Lee toda la entrada de una vez, córtala en el primer salto de línea, interpreta lo de antes como `s` y guarda el resto como un solo bloque.

**Descifrar una letra.** Al cifrar, cada letra avanzó `s` posiciones, así que descifrar es retrocederla `s` posiciones en un ciclo de 26 letras. Para una letra minúscula `c`:

```
p = c - 'a'                 # 0..25
normal = 'a' + (p - s + 26) mod 26
```

y lo mismo con `'A'` para las mayúsculas. El `+ 26` mantiene el valor no negativo en los lenguajes en los que `%` de un número negativo da negativo. Cualquier otro carácter, incluidos los saltos de línea, se copia tal cual, así que las líneas vacías se conservan solas.

**Una tabla en lugar de cuentas.** Solo hay 52 letras, así que se puede construir una vez una tabla de traducción de cada letra cifrada a su letra normal: el alfabeto normal es el alfabeto rotado `s` posiciones. Con `bytes.maketrans` + `translate` en Python y `strtr` en PHP se descifra todo el bloque en una sola llamada, en C.

**Por qué es correcto:** avanzar `s` y retroceder `s` en el mismo ciclo de 26 letras se anulan, las letras nunca cambian de mayúscula a minúscula y todo lo demás, incluida la estructura de líneas, queda intacto.

**Complejidad:** tiempo y memoria O(L) para L caracteres de texto.

**Entrada rápida en cada lenguaje:** `sys.stdin.buffer.read()` con `partition(b"\n")` y `translate` (Python), `readFileSync(0)` como `Buffer`, modificado en su sitio y escrito con `process.stdout.write` (TypeScript), `stream_get_contents(STDIN)` con `strtr` (PHP), `read_to_end` en un `Vec<u8>`, transformado byte a byte y escrito con `write_all` (Rust).
