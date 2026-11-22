Esta vez ningún par tiene que dar `target` exacto. Elige dos números en posiciones
distintas `i < j` y devuelve la suma `nums[i] + nums[j]` **más cercana** a `target`.

Si dos sumas están igual de cerca (una por debajo de `target` y otra por encima),
devuelve la **menor**: en `[1, 5, 9]` con `target = 8` las sumas 6 y 10 están ambas a
distancia 2, así que la respuesta es 6. Devuelve la suma, no las posiciones.

### Ejemplo 1

```
nums = [7, -2, 15, 4], target = 10
Salida: 11
```

7 + 4 = 11 está a 1 de 10; ningún par suma exactamente 10.

### Ejemplo 2

```
nums = [16, -16, -4, -13], target = 23
Salida: 12
```

### Ejemplo 3

```
nums = [-15, -15, 3, -10], target = -1
Salida: -7
```

3 + (−10) = −7 está a 6 de −1; todas las demás sumas quedan más lejos.

### Restricciones

- 2 ≤ nums.length ≤ 10⁵
- −10⁹ ≤ nums[i] ≤ 10⁹
- −2·10⁹ ≤ target ≤ 2·10⁹
