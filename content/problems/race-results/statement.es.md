Inés publica los resultados de la carrera dominical de su club de corredores. Su script ordena a los corredores por tiempo, pero cada vez que dos cruzan la meta a la vez, los socios se quejan de que el orden parece al revés.

Dados los arreglos `names` y `times`, donde el corredor `names[i]` terminó en `times[i]` segundos, devuelve los nombres ordenados del tiempo más rápido al más lento. Los corredores con el mismo tiempo se ordenan por nombre en orden alfabético ascendente (se compara carácter a carácter; los dígitos van antes que las letras, y un nombre va antes que cualquier nombre más largo que empiece por él).

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
names = ["mia", "leo", "ana", "zoe"], times = [312, 298, 312, 305]
Salida: ["leo", "zoe", "ana", "mia"]
```

"ana" y "mia" empatan con 312 segundos, así que "ana" va primero.

### Ejemplo 2

```
names = ["joe", "ben", "eli", "kim"], times = [103, 103, 103, 101]
Salida: ["kim", "ben", "eli", "joe"]
```

### Ejemplo 3

```
names = ["ben", "lea", "fay", "cai"], times = [415, 357, 317, 136]
Salida: ["cai", "fay", "lea", "ben"]
```

### Restricciones

- 1 ≤ names.length ≤ 10⁴
- times.length = names.length
- Cada nombre tiene de 1 a 10 letras minúsculas del alfabeto inglés o dígitos, y todos los nombres son distintos.
- 1 ≤ times[i] ≤ 10⁶
