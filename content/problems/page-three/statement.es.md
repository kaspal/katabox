Un mercado de segunda mano muestra sus anuncios de diez en diez, primero los mejor valorados. Los moderadores ocultan algunos anuncios: la web no los muestra nunca, así que no ocupan sitio en ninguna página.

Un visitante ha llegado a la **página 3**. Ordena los anuncios visibles (`hidden = 0`) por `rating`, de mayor a menor, luego por `title` y luego por `id`, y devuelve los anuncios de la página 3: la página 1 va del 1.º al 10.º anuncio visible y la página 2 del 11.º al 20.º, así que la página 3 va **del 21.º al 30.º**. La página 3 puede tener menos de diez anuncios, o ninguno.

Este es un **problema SQL**: escribe una sola sentencia `SELECT` (SQLite 3.53, la incluida). Las tablas están en el panel del esquema.

## Tablas

- `listings(id, title, rating, hidden)`: `id` es la clave primaria; `title` es el título del anuncio; `rating` es un número entero de estrellas de 0 a 5; `hidden` vale `1` para un anuncio que la web no muestra y `0` para uno que sí muestra.

## Resultado

Devuelve los anuncios de la página 3, con tres columnas:

| columna | valor |
| --- | --- |
| `id` | el `id` del anuncio |
| `title` | su título |
| `rating` | su valoración |

Las filas van en el orden de la página: `rating`, de mayor a menor, luego `title` y luego `id`. Los títulos se comparan como texto simple, así que `kettle` va antes que `kettle coat`.

### Ejemplo 1

`listings` (25 filas, ninguna oculta):

| id | title | rating | hidden |
| --- | --- | --- | --- |
| 18 | bike | 5 | 0 |
| 73 | lamp boots | 1 | 0 |
| 9 | radio kettle | 2 | 0 |
| 33 | radio | 3 | 0 |
| 16 | kettle desk | 2 | 0 |
| 64 | desk coat | 5 | 0 |
| 58 | kettle lamp | 3 | 0 |
| 61 | kettle boots | 5 | 0 |
| 84 | lamp boots | 4 | 0 |
| 49 | kettle coat | 1 | 0 |
| 27 | chair | 5 | 0 |
| 13 | radio bike | 4 | 0 |
| 63 | chair chair | 5 | 0 |
| 4 | lamp | 5 | 0 |
| 50 | boots | 5 | 0 |
| 56 | radio guitar | 5 | 0 |
| 78 | boots | 5 | 0 |
| 1 | boots | 1 | 0 |
| 93 | tent kettle | 2 | 0 |
| 35 | radio coat | 4 | 0 |
| 30 | radio tent | 1 | 0 |
| 76 | kettle | 5 | 0 |
| 14 | kettle | 3 | 0 |
| 41 | desk | 1 | 0 |
| 86 | lamp guitar | 1 | 0 |

Resultado:

| id | title | rating |
| --- | --- | --- |
| 41 | desk | 1 |
| 49 | kettle coat | 1 |
| 73 | lamp boots | 1 |
| 86 | lamp guitar | 1 |
| 30 | radio tent | 1 |

Los diez anuncios con 5 estrellas llenan la página 1. La página 2 recoge los tres de 4, los tres de 3, los tres de 2 y el primero de 1 por título, `boots` (id 1). Los otros cinco con 1 estrella ocupan los puestos 21.º a 25.º, así que la página 3 tiene solo cinco filas.

### Ejemplo 2

`listings` (12 filas, ninguna oculta):

| id | title | rating | hidden |
| --- | --- | --- | --- |
| 8 | boots | 4 | 0 |
| 12 | kettle radio | 5 | 0 |
| 11 | bike | 3 | 0 |
| 47 | boots boots | 5 | 0 |
| 22 | lamp | 2 | 0 |
| 95 | chair chair | 5 | 0 |
| 86 | radio | 4 | 0 |
| 40 | tent coat | 3 | 0 |
| 33 | chair boots | 4 | 0 |
| 78 | radio | 3 | 0 |
| 28 | radio radio | 3 | 0 |
| 5 | radio lamp | 3 | 0 |

Resultado: ninguna fila. Doce anuncios terminan en la página 2.

## Restricciones

- 1 ≤ anuncios ≤ 1000; los ids son distintos, de 1 a 10⁶.
- Un título es de una a tres palabras en minúsculas; varios anuncios pueden tener el mismo título.
- `rating` es un número entero, 0 ≤ `rating` ≤ 5.
- `hidden` vale `0` o `1`.
