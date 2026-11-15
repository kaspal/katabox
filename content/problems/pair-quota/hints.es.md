Probar todas las parejas son `n(n − 1)/2` comprobaciones: unos 5·10⁹ para un caso de 10⁵ equipos. El recuento no depende del orden de los saldos, así que puedes reordenarlos.
---
Ordena los saldos. Si el menor más el mayor caben en `K`, el menor cabe con **todos** los demás saldos, porque todos son como mucho el mayor. Si no caben, el mayor no cabe con ninguno de los que quedan.
---
Dos punteros sobre los saldos ordenados, `i = 0` y `j = n − 1`: mientras `i < j`, si `a[i] + a[j] ≤ K` suma `j − i` al recuento y mueve `i` a la derecha; si no, mueve `j` a la izquierda. Hazlo por separado en cada caso, empezando el recuento en 0, y guárdalo en un entero de 64 bits.
