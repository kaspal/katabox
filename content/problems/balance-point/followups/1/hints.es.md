Probar todos los pares de apoyos son unos 5·10⁹ pares. Con sumas prefijas, cada total
de tramo sale en O(1); ¿puedes evitar también el bucle sobre el primer apoyo?
---
Escribe los tres totales con sumas prefijas `P` (`P[k]` = suma de las `k` primeras
cajas): izquierda = `P[i]`, centro = `P[j] − P[i] − weights[i]`, derecha =
`total − P[j + 1]`. Fija `j` y pregúntate cómo tiene que ser un `i` anterior.
---
Para un `j` fijo, llama `R` al total derecho. Un `i` válido necesita `P[i] = R` y
`weights[i] = P[j] − 2R`. Recorre `j` de izquierda a derecha, añadiendo cada
`i = j − 1` a un mapa con clave el par `(P[i], weights[i])`, y suma la cuenta guardada
bajo `(R, P[j] − 2R)`.
