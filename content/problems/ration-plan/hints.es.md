Para comprar tantos paquetes como sea posible, ¿qué paquetes debería elegir primero un equipo? Y como las existencias son las mismas para todos los equipos, ¿qué podrías preparar una sola vez, antes de leer los presupuestos?
---
Los mejores `k` paquetes que se pueden comprar son siempre los `k` más baratos. Ordena los precios y súmalos empezando por el más barato: los totales acumulados solo crecen, así que la respuesta para un presupuesto es cuántos totales acumulados caben en ese presupuesto.
---
Ordena los precios y construye `total[k]` = la suma de los `k + 1` más baratos (sumas de prefijos). Para cada presupuesto, busca con búsqueda binaria cuántos totales son `≤ presupuesto` (`bisect_right` en Python, `partition_point` en Rust, una búsqueda escrita a mano en TypeScript y PHP). Eso es O(n log n + Q log n).
