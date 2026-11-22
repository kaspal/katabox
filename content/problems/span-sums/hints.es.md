Prueba la consulta más pequeña posible: un solo día, `[l, l]`. ¿Qué debería devolver y qué devuelve el código?
---
La construcción de la lista de prefijos parece correcta. Fíjate en cómo cada consulta convierte `l` y `r` en dos posiciones de `prefix`. ¿Qué guarda realmente `prefix[i]`?
---
`prefix[i]` es el total de los primeros `i` días, es decir, de los días `0` a `i - 1`. Por eso `prefix[r] - prefix[l]` cubre los días `l` a `r - 1` y se deja el día `r`. Usa `prefix[r + 1] - prefix[l]`.
