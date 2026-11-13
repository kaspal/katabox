Multiplicar `exponent` veces no tiene remedio con 10¹⁵. ¿Cómo obtienes b⁸ a partir de b⁴ con una sola multiplicación?
---
Elevando al cuadrado: b⁸ = (b⁴)². Cualquier exponente es una suma de potencias de dos (sus dígitos binarios), por ejemplo 13 = 8 + 4 + 1, así que b¹³ = b⁸ · b⁴ · b.
---
Recorre los bits del exponente desde el más bajo: si el bit es 1, multiplica el resultado por el cuadrado actual; después eleva al cuadrado el cuadrado actual. Toma el resto tras cada multiplicación para que los números sigan siendo pequeños (por debajo de 1 000 003², unos 10¹²).
