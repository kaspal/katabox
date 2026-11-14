## La idea

Un número se guarda como una fila de bits, y cada columna es una potencia de dos: `44` es `00101100`, es decir, 32 + 8 + 4. Los operadores de bits trabajan sobre todas las columnas a la vez, en un solo paso de la máquina. `bit_ops(x, y)` aplica AND, OR, XOR y los dos desplazamientos a dos bytes, y luego cuenta los bits a 1 de `x` con un truco clásico: `x & (x - 1)` borra el 1 más bajo, así que el bucle da una vuelta por cada bit a 1 y no una por cada columna.

## Pasos

1. `x & y`: un 1 solo en las columnas donde los dos tienen un 1.
2. `x | y`: un 1 en las columnas donde alguno tenga un 1.
3. `x ^ y`: un 1 en las columnas donde son distintos.
4. `x << 1`: cada bit pasa una columna a la izquierda, lo que duplica `x`.
5. `x >> 1`: cada bit pasa una columna a la derecha y el último se pierde: `x` a la mitad, redondeado hacia abajo.
6. Mientras `x != 0`: `x &= x - 1` y suma uno a `count`. Restar 1 invierte el 1 más bajo y los 0 que tiene debajo, así que el AND borra justo ese bit.

## Coste

| Operación | Coste |
|---|---|
| AND, OR, XOR, desplazamientos | `O(1)`: un paso de la máquina sobre una palabra entera |
| Contar los bits a 1 | `O(número de bits a 1)`, como mucho el tamaño de la palabra |
| Espacio extra | `O(1)` |

## Errores comunes

- **La precedencia de los operadores.** En JavaScript, `x & 1 === 0` significa `x & (1 === 0)`, porque la comparación se agrupa antes que `&`. Escribe siempre `(x & 1) === 0`. Python lo agrupa al revés, lo que hace fácil perder la costumbre.
- **JavaScript trabaja con 32 bits.** Los operadores de bits convierten los números en enteros de 32 bits con signo: `1 << 31` es negativo y los valores grandes pierden sus bits altos. Usa `>>>` para un desplazamiento a la derecha sin signo, o `BigInt` por encima de 32 bits.
- **Números negativos en Python.** Los enteros de Python no tienen ancho fijo, y uno negativo se comporta como si tuviera infinitos bits a 1 a la izquierda, así que `while x != 0: x &= x - 1` no termina nunca con `x < 0`. Aplica antes una máscara, por ejemplo `x & 0xFF`.

## Cuándo usarlo

Usa bits para guardar un conjunto de indicadores pequeños en un solo entero (el bit `i` a 1 significa que el elemento `i` está dentro), para consultar o cambiar un indicador en `O(1)`, para recorrer todos los subconjuntos de hasta unos 20 elementos como los números `0 .. 2ⁿ − 1`, y para trucos como hacer XOR de una lista para encontrar el único valor sin pareja. `(x & (x - 1)) == 0` también comprueba si un `x` positivo es potencia de dos.

## Temas relacionados

- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): el bucle de subconjuntos cuenta hasta `2ⁿ`, una máscara de bits por subconjunto.
- [Sumas prefijas](learn:foundations/prefix-sums): otro truco pequeño que convierte trabajo repetido en `O(1)`.
- Programación dinámica con máscaras de bits (próximamente).
