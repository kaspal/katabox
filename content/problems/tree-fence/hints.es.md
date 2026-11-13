Solo importan los árboles que forman las "esquinas" de la cerca. Si los tuvieras en orden alrededor de la cerca, la fórmula del lazo (shoelace) daría directamente el doble del área.
---
Ordena los árboles por x (y después por y). Al recorrerlos de izquierda a derecha, la mitad inferior de la cerca solo gira a la izquierda; un árbol que la haría girar a la derecha (o seguir recta) no puede ser una esquina y se descarta.
---
Cadena monótona de Andrew: construye la envolvente inferior de izquierda a derecha y la superior de derecha a izquierda, quitando la última esquina mientras `cross(penúltima, última, nuevo) ≤ 0`. Después suma `x_i·y_{i+1} − x_{i+1}·y_i` alrededor de la envolvente.
