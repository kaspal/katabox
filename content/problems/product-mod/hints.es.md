Los ejemplos pequeños pasan. Prueba valores cercanos a 10⁹, por ejemplo `nums = [999999999, 999999999]`, y compara con la respuesta exacta.
---
El producto acumulado se reduce módulo 10⁹ + 7 en cada paso, así que se mantiene por debajo de unos 10⁹. Pero la multiplicación ocurre antes de la reducción: ¿qué tan grande puede ser `acc * x` y en qué tipo de número se guarda?
---
`acc` es un número de coma flotante, y un double solo representa enteros exactos hasta 2⁵³ (unos 9·10¹⁵). `acc * x` puede llegar a unos 10¹⁸, así que se redondea antes del `% MOD`. Deja `acc` como entero (`acc = 1`). En TypeScript, `number` tiene el mismo límite: haz la multiplicación con `BigInt` y convierte de vuelta al final.
