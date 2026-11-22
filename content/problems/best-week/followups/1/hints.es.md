Con pérdidas, un tramo más largo no siempre es mejor, así que una ventana fija de `k` días
ya no sirve. Probar cada longitud de 1 a `k` en cada final es O(n · k). ¿Cómo se escribe la
suma de un tramo con sumas prefijas?
---
El tramo que empieza en el día `i` y termina en el día `j − 1` suma
`prefix[j] − prefix[i]`. Para un final fijo, buscas el menor `prefix[i]` con
`j − k ≤ i ≤ j − 1`: un mínimo en ventana deslizante.
---
Guarda una cola doble de índices de prefijos cuyos valores crecen del frente al fondo.
Para cada final `j`, mete `j − 1` tras sacar por el fondo los valores mayores o iguales,
quita el frente si es anterior a `j − k`, y compara `prefix[j] − prefix[frente]` con el
mejor.
