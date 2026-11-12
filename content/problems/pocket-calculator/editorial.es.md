## Enfoque: descenso recursivo (la pila de llamadas es la pila)

Escribe la gramática con una regla por nivel de precedencia:

```
expr   := term   (("+" | "-") term)*
term   := factor ("*" factor)*
factor := "-" factor | número | "(" expr ")"
```

Cada regla se convierte en una función que lee desde una posición compartida de la cadena, saltando espacios:

```
expr():   v = term();   mientras lo siguiente sea + o -: op = leer; w = term(); v = v + w o v − w
term():   v = factor(); mientras lo siguiente sea *: leer; v = v * factor()
factor(): si lo siguiente es "-": leer; devolver −factor()
          si lo siguiente es "(": leer; v = expr(); leer ")"; devolver v
          leer los dígitos; devolver el número
```

**Por qué es correcto.** `term` termina todas las multiplicaciones antes de que `expr` vea el siguiente `+` o `-`, así que `*` liga más fuerte. Los bucles `while` acumulan desde la izquierda, así que `10 − 2 − 3` es `(10 − 2) − 3`. Un menos unario es un factor, así que `2 * -3` y `-(…)` se analizan de forma natural, y `--1` es `−(−1)`.

La pila de llamadas guarda los operadores pendientes en lugar de una pila de operadores explícita (el algoritmo shunting-yard también sirve). Con el anidamiento limitado a 100 niveles, la recursión se mantiene pequeña.

### Paso a paso

`-(7 - 10) * -2`: `expr` → `term` → `factor` ve `-`, así que niega el factor siguiente: `(7 − 10)` = −3, que da 3. De vuelta en `term`, `*` lee `factor` = −2. El resultado es −6.

**Complejidad:** O(n) de tiempo, cada carácter se lee una vez, y O(profundidad) de pila.
