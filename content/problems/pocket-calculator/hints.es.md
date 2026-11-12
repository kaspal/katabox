Leyendo de izquierda a derecha no puedes aplicar `+` en cuanto lo ves: un `*` posterior puede ligar más fuerte. Algo tiene que esperar, y para eso está una pila.
---
Describe la expresión con tres reglas: una *expresión* son términos unidos por `+`/`-`; un *término* son factores unidos por `*`; un *factor* es un número, un `-` seguido de un factor o una expresión entre paréntesis. Cada regla se convierte en una función que llama a las demás (descenso recursivo; la pila de llamadas hace la espera).
---
Guarda una posición en la cadena y salta los espacios antes de leer cada símbolo. `term()` lee un factor y sigue multiplicando mientras vea `*`; `expr()` lee un término y sigue sumando o restando mientras vea `+` o `-`, lo que aplica los operadores del mismo nivel de izquierda a derecha. `factor()` se ocupa del número, del menos unario y de `(` expr `)`.
