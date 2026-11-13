Divide la ruta por `/` y trata las partes una a una. ¿Qué necesitas recordar sobre dónde estás?
---
Solo la lista de carpetas desde la superior hasta la actual. Entrar en una carpeta añade una al final; `..` quita la última.
---
Usa una pila: salta las partes vacías y `.`; con `..`, saca un elemento si la pila no está vacía; si no, mete el nombre. Une la pila con `/` y pon una `/` delante.
