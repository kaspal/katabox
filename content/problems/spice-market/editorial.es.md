## Enfoque: primero el mejor valor por gramo (mochila fraccionaria)

Como los sacos se pueden partir, la bolsa son solo `capacity` gramos que llenar, y cada gramo debe llevar la especia más valiosa que quede. Así que se toman las especias por orden decreciente de valor por gramo:

```
ordenar puestos por prices[i] / weights[i], de mayor a menor   # comparar p1·w2 > p2·w1
total = 0; sitio = capacity
para cada puesto (w, p):
    si w <= sitio: total += p; sitio -= w
    si no: total += p * sitio / w; parar
```

**Por qué es óptimo.** Supón que una carga mejor lleva algunos gramos de una especia `b` mientras una especia `a` con más valor por gramo no se ha tomado entera. Cambiar uno de esos gramos de `b` por un gramo de `a` mantiene el peso y no baja el valor. Repitiendo el cambio, cualquier carga se convierte en la voraz sin perder valor.

Solo se parte el último saco, así que la respuesta es un número entero más una fracción. Comparar las razones multiplicando en cruz (`p1 · w2` contra `p2 · w1`, como mucho 10⁸) evita redondeos al ordenar.

### Paso a paso

Valores por gramo 6, 5, 4. Se toman 10 g (60) y 20 g (100); quedan 20 g de sitio, así que se toman 20/30 del tercer saco: 120 · 20 / 30 = 80. Total 240.

**Complejidad:** O(n log n) para ordenar y O(n) para el recorrido.
