Un mapa hash encuentra una pareja exacta, pero aquí la mejor pareja puede no existir.
¿En qué orden tendrían que estar los números para llegar fácilmente a "una suma un poco
mayor" o "un poco menor"?
---
Ordena los números y mira juntos el menor y el mayor. Si su suma queda por debajo de
`target`, ¿puede el menor mejorar con alguna otra pareja?
---
Pon un puntero en cada extremo del array ordenado. Guarda la suma cuando mejore a la
mejor hasta ahora (más cerca, o igual de cerca y menor); luego avanza el puntero
izquierdo si la suma es pequeña, retrocede el derecho si es grande, y para si das con
`target` exacto.
