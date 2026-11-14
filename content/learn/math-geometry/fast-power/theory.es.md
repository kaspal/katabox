## La idea

Para calcular `base^exp mod m`, el bucle obvio multiplica por `base` `exp` veces. Con un exponente de mil millones, son mil millones de multiplicaciones. **Elevar al cuadrado y multiplicar** solo necesita unas dos por cada **bit** del exponente.

Escribe el exponente en binario. `13 = 1101₂ = 8 + 4 + 1`, así que `x^13 = x^8 · x^4 · x^1`. Las potencias `x, x², x⁴, x⁸, …` salen baratas: cada una es el **cuadrado** de la anterior. Así que el bucle recorre los bits del exponente desde el más bajo: si el bit actual es 1, **multiplica** el resultado por la potencia actual; después eleva la potencia al cuadrado y desplaza el exponente un bit a la derecha. Cuando el exponente llega a 0, cada bit a 1 ha aportado su potencia.

La segunda idea es **reducir mod `m` tras cada multiplicación**. `(x · y) mod m` es igual a `((x mod m) · (y mod m)) mod m`, así que tomar el resto pronto nunca cambia la respuesta, y todos los valores se quedan por debajo de `m`. Sin eso, `7^1000` tiene 846 cifras; con eso, ningún producto pasa de `(m - 1)²`.

El ejemplo empieza con `result = 1 mod m` (así un módulo 1 da 0) y `base mod m`. La salida es `[result]`. Los tres números se escriben en tres campos: base, exponente y módulo.

### La entrada por defecto

`7^1000 mod 13` (el ajuste "un exponente grande"). En binario, `1000 = 1111101000₂`: diez bits, seis de ellos a 1. La potencia actual `7^(2^i) mod 13` vale 7, 10, 9, 3 y después alterna 9, 3, 9, 3, 9, 3, un valor por bit.

Los tres bits más bajos son 0, así que el bucle solo eleva al cuadrado. El bit 3 es el primer 1: el resultado pasa a 3. El bit 4 es 0; los bits 5 a 9 son todos 1, y el resultado alterna 9, 3, 9, 3, 9. La salida es `[9]`, tras 6 multiplicaciones y 10 cuadrados, en lugar de 1000 multiplicaciones.

## Pasos

1. Pon `result = 1 mod m` y `base = base mod m`.
2. Mientras `exp > 0`: si su bit más bajo es 1, haz `result = result · base mod m`.
3. Haz `base = base · base mod m` y desplaza `exp` un bit a la derecha.
4. Devuelve `result`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(log exp)`: un cuadrado por bit y como mucho una multiplicación |
| Espacio extra | `O(1)` |

El ajuste "una potencia de dos", `3^64 mod 101`, muestra la forma más barata: `64 = 1000000₂` tiene un único bit a 1, así que 7 cuadrados y una sola multiplicación dan `[88]`.

## Errores comunes

- **Reducir solo al final.** La potencia intermedia se sale de cualquier tipo entero casi de inmediato.
- **Desbordamiento del producto.** Dos valores menores que `m` dan un producto de casi `m²`; con un módulo cercano a 10⁹ hacen falta enteros de 64 bits (y con uno cercano a 10¹⁸, de 128).
- **El módulo 1.** Todo vale 0 mod 1; empezar el resultado en un 1 a secas devolvería 1.
- **Un exponente cero.** El bucle no se ejecuta y el resultado es `1 mod m`, también para `0⁰`.

## Cuándo usarlo

Úsalo siempre que se pida una potencia enorme módulo algo: hashing, contar respuestas "módulo un primo" o comprobar el ciclo de una potencia. El problema enlazado, Potencia rápida, pide justo `base^exponent mod 1 000 003` con un exponente de hasta 10¹⁵. El mismo bucle da también inversos modulares mediante el pequeño teorema de Fermat.

## Temas relacionados

- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): leer el bit más bajo y desplazar a la derecha.
- [Inverso modular](learn:math-geometry/modular-inverse): con un módulo primo, el inverso es una potencia rápida.
- [Recursión y pila de llamadas](learn:foundations/recursion-call-stack): la misma idea escrita de forma recursiva: para un `e` par, `x^e = (x^(e/2))²`.
- [Hash rodante](learn:hashing/rolling-hash): potencias de una base módulo un número, paso a paso.
