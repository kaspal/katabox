Un hackathon necesita un equipo que cubra `skills` habilidades distintas, numeradas de 0 a `skills − 1`. Las habilidades de cada voluntario se guardan como un número: el bit `s` de `people[i]` es 1 cuando el voluntario `i` tiene la habilidad `s`.

Devuelve el tamaño del equipo más pequeño cuyos miembros, juntos, tienen todas las habilidades, o `-1` si ni siquiera todos juntos las tienen.

### Ejemplo 1

```
skills = 4, people = [3, 4, 12, 9]
Salida: 2
```

3 es 0011 (habilidades 0 y 1) y 12 es 1100 (habilidades 2 y 3).

### Ejemplo 2

```
skills = 3, people = [1, 2]
Salida: -1
```

Nadie tiene la habilidad 2.

### Ejemplo 3

```
skills = 2, people = [3]
Salida: 1
```

### Restricciones

- 1 ≤ skills ≤ 14
- 1 ≤ people.length ≤ 60
- 0 ≤ people[i] < 2^skills
