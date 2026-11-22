Con muchas etiquetas, el trabajo que haces una vez por estantería importa menos que el
que haces por etiqueta. ¿Hay algo de la estantería que valga la pena encontrar una sola
vez?
---
Encuentra dónde empieza el orden original: la posición del número más pequeño. Comparar
el número del centro con el último te dice en qué mitad está.
---
Cuando sepas el inicio `s`, la estantería leída desde `s` hasta el final y luego desde 0
hasta `s − 1` está totalmente ordenada. Haz una búsqueda binaria de cada etiqueta en esa
vista ordenada y convierte un índice encontrado `k` en posición de la estantería con
`(k + s) mod n`.
