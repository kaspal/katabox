Un viejo servidor de archivo recibe rutas de carpetas escritas a mano, y a menudo vienen desordenadas. Una ruta empieza por `/` (la carpeta superior) y `/` la divide en partes:

- una parte vacía (de `//` o de una `/` final) y la parte `.` significan "quédate aquí";
- la parte `..` significa "sube a la carpeta padre"; en la carpeta superior se queda arriba;
- cualquier otra parte (letras, dígitos, `_` y puntos, como `...` o `.config`) es un nombre de carpeta: entra en ella.

Devuelve la ruta limpia: `/` seguida de los nombres de las carpetas en las que terminas, separados por una sola `/`, sin `/` final. La carpeta superior es `/`.

### Ejemplo 1

```
path = "/photos/./2019//summer/../winter/"
Salida: "/photos/2019/winter"
```

### Ejemplo 2

```
path = "/../.."
Salida: "/"
```

### Ejemplo 3

```
path = "/notes/...//a"
Salida: "/notes/.../a"
```

`...` es un nombre de carpeta, no un movimiento.

### Restricciones

- 1 ≤ path.length ≤ 3000
- path empieza por `/` y solo tiene letras, dígitos, `.`, `_` y `/`.
