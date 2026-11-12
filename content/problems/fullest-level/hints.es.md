Necesitas el total de cada nivel. ¿Puedes recorrer el árbol nivel a nivel?
---
Una cola recorre los nodos por niveles. Si sabes cuántos nodos tiene el nivel actual antes de empezarlo, sabes dónde termina.
---
Procesa la cola por rondas: lee su longitud, saca esa cantidad de nodos (sumando sus valores al total del nivel y metiendo sus hijos) y compara el total con el mejor hasta ahora. Solo un total estrictamente mayor cambia la respuesta, así que los empates conservan el nivel menos profundo.
