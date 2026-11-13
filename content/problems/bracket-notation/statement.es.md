Un club de genealogía guarda pequeños árboles familiares binarios en una sola línea de texto. Un nodo se escribe como su valor, seguido de su subárbol izquierdo entre paréntesis y después su subárbol derecho entre paréntesis, con estas reglas para acortar el texto:

- un nodo sin hijos se escribe solo con su valor;
- un hijo **derecho** que falta se omite, así que un nodo con solo hijo izquierdo es `valor(izquierdo)`;
- un hijo **izquierdo** que falta se escribe `()` cuando hay hijo derecho, para que el derecho no se confunda con un izquierdo: `valor()(derecho)`.

Devuelve el texto de `root`, o la cadena vacía para un árbol vacío.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [4, 2, 7, 1]
Salida: "4(2(1))(7)"
```

### Ejemplo 2

```
root = [4, null, 7, 5]
Salida: "4()(7(5))"
```

### Ejemplo 3

```
root = [-3]
Salida: "-3"
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- -1000 ≤ valor de un nodo ≤ 1000
- El árbol tiene como mucho 1000 niveles.
