Comprobar cada trozo por separado cuesta O(n) por trozo y O(n³) en total. Cada trozo espejo tiene un centro. ¿Qué pasa si empiezas por el centro?
---
Desde un centro, extiende una letra a la izquierda y una a la derecha mientras sean iguales. Un centro es una letra (longitud impar) o el hueco entre dos letras (longitud par): 2n − 1 centros en total.
---
Expande alrededor de cada uno de los 2n − 1 centros en orden de izquierda a derecha, quedándote con el trozo más largo; sustitúyelo solo por uno **estrictamente** más largo, para que los empates conserven el que empieza antes.
