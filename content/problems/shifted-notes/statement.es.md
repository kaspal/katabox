En un campamento de verano, los monitores organizan una búsqueda nocturna de pistas. Para que nadie lea las pistas de un vistazo, las cifran: cada letra avanza `s` posiciones en el alfabeto, volviendo de la `z` a la `a` (y de la `Z` a la `A`). Con `s = 3`, la `a` pasa a ser `d`, la `y` pasa a ser `b` y la `M` pasa a ser `P`; los dígitos, los espacios y los signos de puntuación no cambian. La directora del campamento tiene la clave y quiere un programa que devuelva un montón de pistas cifradas a texto normal.

Este es un **problema de programa**: lee la clave y el texto cifrado de la entrada estándar y escribe el texto descifrado en la salida estándar.

## Formato de entrada

- La primera línea contiene `s`, el desplazamiento que se usó para cifrar el texto.
- Todas las líneas siguientes, hasta el final de la entrada, son líneas de texto cifrado. Puede haber líneas vacías. No se indica cuántas líneas hay: lee hasta que se acabe la entrada.

## Formato de salida

Escribe cada línea de texto descifrada, en orden: cada letra **retrocede** `s` posiciones dentro de su propio tipo, mayúscula o minúscula (volviendo de la `a` a la `z` y de la `A` a la `Z`), y cualquier otro carácter queda igual. Las líneas vacías siguen siendo líneas vacías.

La salida se compara línea a línea, exactamente salvo los espacios al final.

### Ejemplo 1

Entrada:

```
3
Phhw dw wkh Rog Rdn dw 5.
Eulqj d odps!
```

Salida:

```
Meet at the Old Oak at 5.
Bring a lamp!
```

### Ejemplo 2

Entrada:

```
25
Ydaqz bqnrrhmf, ynmd Z.

Qnnl 42 hr nodm.
```

Salida:

```
Zebra crossing, zone A.

Room 42 is open.
```

Retroceder 25 posiciones es lo mismo que avanzar 1. La línea vacía se queda en su sitio.

### Ejemplo 3

Entrada:

```
0
No secrets here: 100% plain!
```

Salida:

```
No secrets here: 100% plain!
```

### Restricciones

- 0 ≤ s ≤ 25
- Hay al menos una línea de texto.
- El texto que sigue a la primera línea tiene como mucho 150 000 caracteres, cada uno un carácter ASCII imprimible (letras, dígitos, espacios, signos de puntuación) o un salto de línea.
