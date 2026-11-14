## La idea

Un arreglo dinámico (una `list` de Python, un arreglo de JavaScript) guarda sus valores en un bloque de almacenamiento con sitio para `capacity` valores, de los que `size` están en uso. Añadir un valor escribe en el siguiente hueco libre. Cuando el bloque está lleno no hay sitio para crecer donde está, así que el arreglo reserva un bloque nuevo con el **doble** de capacidad, copia todos los valores y solo entonces escribe el nuevo. Esa copia es lenta, pero ocurre tan pocas veces que el coste **promediado entre todas las inserciones** se mantiene constante: es un `O(1)` amortizado.

## Pasos

1. Empieza con `capacity = 1`, `size = 0` y `copies = 0`.
2. Para cada valor `v`, comprueba: ¿es `size == capacity`?
3. Si está lleno: dobla `capacity`, reserva el almacenamiento más grande y copia los `size` valores, una copia cada uno.
4. Escribe `v` en la posición `size` y suma uno a `size`.
5. Mira el contador "escrituras por inserción": escrituras más copias, divididas entre las inserciones hechas hasta ahora.

## Coste

| Medida | Coste |
|---|---|
| Una inserción, normalmente | `O(1)`: una sola escritura |
| Una inserción que redimensiona | `O(n)`: se copian todos los valores |
| `n` inserciones en total | `O(n)`: menos de `2n` copias más `n` escrituras |
| Amortizado por inserción | `O(1)`: siempre menos de 3 escrituras |
| Espacio extra | `O(n)`: como mucho la mitad del almacenamiento está vacía |

## Errores comunes

- **Crecer una cantidad fija.** Añadir, por ejemplo, 10 huecos en cada redimensión copia `10 + 20 + 30 + …` valores: `O(n²)` en total, `O(n)` por inserción. El crecimiento tiene que ser un **factor** (las bibliotecas reales eligen uno como 1,5 o 2), para que cada copia se pague con muchas inserciones baratas.
- **Amortizado no es "siempre rápido".** Una inserción puede seguir copiando todos los valores. En código con un plazo por operación, ese pico aislado puede importar; si conoces el tamaño final, resérvalo desde el principio.
- **Encoger con demasiadas prisas.** Reducir el almacenamiento a la mitad en cuanto está medio vacío hace que alternar una inserción y una eliminación justo en el límite redimensione cada vez. Encoge solo cuando el arreglo esté lleno a un cuarto.

## Cuándo usarlo

Por esto añadir a una lista dentro de un bucle está bien: `n` inserciones cuestan `O(n)`, no `O(n²)`. En general, el análisis amortizado te permite aceptar un paso caro de vez en cuando si los baratos lo pagan, el mismo razonamiento que hay detrás de las tablas hash que se redimensionan y de las colas hechas con dos pilas.

## Temas relacionados

- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): `O(1)`, `O(n)` y `O(n²)`, uno al lado del otro.
- [Sumas prefijas](learn:foundations/prefix-sums): otro caso de pagar una vez al principio para que el trabajo posterior salga barato.
- Tablas hash (próximamente): se redimensionan de la misma manera.
