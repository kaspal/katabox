Prueba una ruta en la que todos los controles tengan la misma altura, como `[2, 2]`. La respuesta debería ser 1.
---
La idea es buena: `tails[k]` es la menor altura final de una elección ascendente de longitud `k + 1`, y cada altura o bien alarga `tails` o bien reemplaza un elemento. Mira la búsqueda binaria que elige la posición: ¿qué hace cuando `h` ya está en `tails`?
---
`bisect_right` devuelve la posición que queda después de los valores iguales, así que una altura repetida cae detrás de su gemela y alarga la racha, con lo que cuenta elecciones no decrecientes. Usa `bisect_left` para que una altura igual reemplace a su gemela (en TypeScript, cambia `tails[mid] <= h` por `tails[mid] < h`).
