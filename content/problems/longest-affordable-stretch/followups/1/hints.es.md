La ventana deslizante del problema base dependía de que los costes nunca fueran negativos:
alargar un tramo nunca lo abarataba. Con reembolsos, un tramo demasiado caro puede volver a
ser asequible más adelante. ¿Pueden ayudar las sumas prefijas?
---
Con `prefix[j]` el coste de las primeras `j` etapas, el tramo `i..j−1` cabe cuando
`prefix[i] ≥ prefix[j] − budget`. Para cada final `j`, buscas el `i` **más temprano** que
lo cumpla. ¿Qué inicios pueden llegar a ser el más temprano?
---
Un inicio solo merece la pena si su prefijo supera a todos los anteriores: un inicio
anterior al menos igual de alto siempre es mejor. Esos máximos forman una lista creciente;
para cada `j`, busca en binario el primer valor ≥ `prefix[j] − budget` y después añade
`prefix[j]` a la lista si es un nuevo máximo.
