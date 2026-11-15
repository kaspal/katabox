## Enfoque: dividir, sumar en un mapa y ordenar los nombres

**Lectura.** Las etiquetas no tienen espacios, así que toda la entrada dividida por espacios es `N` seguido de las `N` etiquetas, un token cada una. Separa cada etiqueta en dos pasos:

```
"pears:5;apples:1;figs:40"
  dividir por ";"  →  "pears:5", "apples:1", "figs:40"
  dividir por ":"  →  ("pears", 5), ("apples", 1), ("figs", 40)
```

**Suma.** Lleva un mapa `total` del nombre a la cantidad. Para cada entrada, `total[name] = total.get(name, 0) + quantity`. Un nombre repetido en la misma etiqueta simplemente se suma dos veces, igual que uno repetido en etiquetas distintas.

**Orden.** Ordena los nombres del mapa y escribe `name total` para cada uno. Los nombres solo tienen letras minúsculas, así que la comparación normal de cadenas de cualquier lenguaje es el orden alfabético que pide el enunciado: compara letra a letra y, cuando un nombre se acaba antes (`fig` frente a `figs`), el más corto va primero.

**Por qué es correcto:** cada entrada de cada etiqueta suma su cantidad a exactamente un total, el de su nombre, así que cada total es la suma de todas las cantidades de ese producto. Cada producto aparece una sola vez en el mapa, así que se escribe una vez, y la ordenación pone las líneas en el orden pedido.

**Complejidad:** como mucho 1000 · 20 = 2·10⁴ entradas, cada una O(1) de media en el mapa, y luego O(P log P) para ordenar los P nombres distintos: O(E + P log P) en total, para E entradas. El mayor total es 2·10⁴ · 1000 = 2·10⁷, que cabe en cualquier tipo entero.

**Trampas de cada lenguaje:**

- PHP: `explode(";", $label)` y `explode(":", $entry)`. Acumula los totales en un arreglo indexado por nombre y ordénalo con `ksort($total, SORT_STRING)`, que deja claro que las claves se comparan como cadenas.
- TypeScript: un `Map<string, number>`; `[...total.keys()].sort()` sin comparador da aquí el orden correcto (compara por código de carácter).
- Rust: un `HashMap<&str, u32>` que se ordena al final, o un `BTreeMap`, que mantiene los nombres en orden al insertarlos.

**Entrada y salida rápidas:** lee toda la entrada estándar de una vez y escribe todas las líneas de una sola vez.
