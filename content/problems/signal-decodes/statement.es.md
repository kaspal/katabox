Un equipo de rescate de montaña lleva todavía una vieja radio de campaña para los lugares sin cobertura. Envía cada letra como su número en el alfabeto, `A` = 1, `B` = 2, …, `Z` = 26 (sin la Ñ), pero no envía separadores, así que un mensaje recibido es solo una tira de dígitos y muchas veces se puede leer de varias maneras. Por ejemplo, `25` puede ser `BE` (2, 5) o `Y` (25).

El receptor anota cada mensaje en su propia línea, con un signo `#` seguido de los dígitos. Antes de descifrar nada, el equipo quiere saber cuán ambiguo es cada mensaje: ¿de cuántas formas se puede partir en números del 1 al 26? Un trozo nunca empieza por cero, así que ni `0` suelto ni `05` son letras, mientras que `10` y `20` sí lo son.

Este es un **problema de programa**: lee el registro de la entrada estándar y escribe los recuentos en la salida estándar.

## Formato de entrada

- La primera línea contiene `T`, el número de mensajes.
- Cada una de las `T` líneas siguientes contiene un mensaje: un `#` seguido de sus dígitos.

## Formato de salida

Escribe `T` líneas, una por mensaje y en orden: el número de formas de leerlo como una secuencia de letras, **módulo 1 000 003**. Un mensaje que no se puede leer de ninguna forma escribe `0`.

### Ejemplo 1

Entrada:

```
3
#118
#2025
#9
```

Salida:

```
3
2
1
```

`118` se lee como 1 1 8, 11 8 o 1 18, y `2025` como 20 2 5 o 20 25.

### Ejemplo 2

Entrada:

```
4
#10
#2101
#30
#0
```

Salida:

```
1
1
0
0
```

`2101` solo se lee como 2 10 1: el `0` tiene que unirse al `1` anterior. En `30` el `0` no puede unirse al `3`, porque 30 no es una letra.

### Ejemplo 3

Entrada:

```
2
#111111111111111111111111111111
#2626
```

Salida:

```
346266
4
```

Los treinta dígitos 1 se pueden leer de 1 346 269 formas, que módulo 1 000 003 son 346 266.

### Restricciones

- 1 ≤ T ≤ 1000
- Cada mensaje tiene entre 1 y 10⁵ dígitos (`0`-`9`).
- Como mucho 2·10⁵ dígitos entre todos los mensajes.
