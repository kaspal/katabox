## La idea

Un **filtro de Bloom** responde a una sola pregunta, "¿he visto esta clave?", usando mucha menos memoria que guardar las claves. Es una fila de `m` bits, todos a 0 al principio, más `k` funciones hash que llevan cada clave a un bit. Este tiene 16 bits y tres hashes de la forma `(a · key + b) mod 16`, con `(a, b)` igual a `(1, 0)`, `(7, 3)` y `(11, 5)`.

- **add** una clave: calcula sus tres posiciones y pon esos bits a 1.
- **check** una clave: mira sus tres bits. Si alguno es 0, la clave **seguro que no** se añadió, porque añadirla habría puesto ese bit a 1. Si los tres son 1, la respuesta es solo **quizá**: otras claves pueden haber puesto esos bits.

### Falsos positivos, nunca falsos negativos

El filtro nunca guarda claves, solo bits que las claves comparten. En la entrada por defecto, add 7 pone los bits 7, 4 y 2, y add 12 pone los bits 12, 7 y 9 (el 7 ya estaba puesto). Después check 23 calcula los bits 7, 4 y 2, justo los de 7, y dice "quizá" aunque 23 nunca se añadió: un **falso positivo**. check 5 para enseguida, porque su primer bit, el 5, está a 0: seguro que no.

Un "no" siempre es correcto, así que no hay falsos negativos. El precio es que un filtro no puede **borrar**: poner un bit a 0 podría borrar parte de otra clave y convertir un "quizá" correcto en un "no" falso.

Cuanto más lleno está el filtro, más a menudo una clave ajena encuentra todos sus bits a 1 por casualidad. Unos 10 bits por clave dan más o menos un 1 % de falsos positivos.

## Pasos

1. Empieza con 16 bits, todos a 0.
2. add `key`: para cada uno de los tres hashes, pon a 1 el bit `(a · key + b) mod 16`.
3. check `key`: para cada hash por turno, lee el bit. Si es 0, devuelve 0 (seguro que no) y para.
4. Si los tres bits son 1, devuelve 1 (quizá).

## Coste

| Operación | Coste |
|---|---|
| add | `O(k)` para `k` hashes, sea cual sea el número de claves |
| check | `O(k)`, a menudo menos: para en el primer bit a 0 |
| Espacio extra | `O(m)` bits, mucho menos que las propias claves |

## Errores comunes

- **Tomar "quizá" por "sí".** Un quizá hay que confirmarlo con los datos reales cuando la respuesta importa.
- **Quitar claves poniendo bits a 0.** Crea falsos negativos. Para borrar hace falta un filtro de Bloom con contadores, que usa pequeños contadores en lugar de bits.
- **Hacerlo demasiado pequeño.** Cuando casi todos los bits están a 1, casi todos los check dicen quizá y el filtro deja de filtrar.

## Cuándo usarlo

Pon un filtro de Bloom delante de una búsqueda cara que casi siempre falla: una base de datos lo consulta antes de leer un bloque del disco, un navegador antes de preguntar a un servidor si un enlace es peligroso, un rastreador web antes de volver a visitar una página. Cada "seguro que no" se ahorra la búsqueda lenta.

## Temas relacionados

- [Tabla hash: encadenamiento](learn:hashing/separate-chaining): respuestas exactas, a cambio de guardar cada clave.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): poner y comprobar bits sueltos.
- [Tabla hash: direccionamiento abierto](learn:hashing/open-addressing): otro arreglo de tamaño fijo guiado por hashes.
