## La idea

Una tabla hash guarda claves de forma que encontrar una no obligue a mirarlas todas. Una **función hash** convierte cada clave en un número de cubeta, y la clave se guarda en esa cubeta. Aquí la tabla tiene 7 cubetas y el hash es simplemente `key mod 7`: la clave 31 va a la cubeta 3 y la 22 a la cubeta 1.

Dos claves distintas pueden caer en la misma cubeta; eso es una **colisión**. Con el **encadenamiento**, cada cubeta es una pequeña lista (una cadena) y las claves que colisionan simplemente la comparten. Para insertar una clave, la tabla calcula su hash, recorre esa única cadena buscando un duplicado y añade la clave solo si es nueva.

### Factor de carga

¿Cuánto crecen las cadenas? Con `n` claves repartidas en `m` cubetas, la cadena media tiene `n / m` claves: el **factor de carga**. La entrada por defecto mete 9 claves en 7 cubetas, un factor de carga de 1,3 más o menos, y aun así la cubeta 3 acaba con tres claves (10, 31 y 17) mientras las cubetas 2 y 4 quedan vacías. Un buen hash reparte las claves por igual; uno malo, o unas claves con mala suerte, las amontona en pocas cadenas. En el peor caso todas comparten una cubeta y la tabla no es más que una lista lenta.

Las tablas reales mantienen bajo el factor de carga **redimensionando**: pasado un límite (a menudo alrededor de 1), duplican las cubetas y recalculan el hash de cada clave, un paso `O(n)` tan raro que sale a `O(1)` amortizado por inserción.

## Pasos

1. Empieza con 7 cubetas vacías.
2. Para cada clave, calcula su cubeta, `b = key mod 7`.
3. Recorre la cubeta `b` buscando la clave. Si ya está, sáltala.
4. Si no, añade la clave al final de la cubeta `b`.
5. La salida lista cada cubeta en orden: su longitud y luego sus claves.

## Coste

| Operación | Coste |
|---|---|
| Insertar o buscar, en promedio | `O(1 + n/m)`: un hash y luego una cadena corta |
| Insertar o buscar, peor caso | `O(n)`: todas las claves en una cadena |
| Borrar | Igual que buscar, y luego quitar de la cadena |
| Espacio extra | `O(n + m)`: las cubetas más una entrada de lista por clave |

## Errores comunes

- **Olvidar buscar el duplicado.** Si añades sin mirar antes, guardas la misma clave dos veces, y un borrado posterior puede quitar solo una copia.
- **Un hash que ignora parte de la clave.** Si haces el hash de una cadena solo con su primera letra, todas las palabras que empiezan por "s" van a la misma cubeta. Usa la clave entera.
- **No redimensionar nunca.** Con un número fijo de cubetas, las cadenas crecen al ritmo de las claves y las búsquedas se vuelven `O(n)`.

## Cuándo usarlo

Usa una tabla hash siempre que necesites "¿está esta clave?" o "¿qué hay guardado para esta clave?" en tiempo constante en promedio: contar palabras, agrupar anagramas, recordar valores ya vistos. `dict` y `set` en Python y `Map` y `Set` en JavaScript ya lo hacen por ti.

## Temas relacionados

- [Tabla hash: direccionamiento abierto](learn:hashing/open-addressing): sin listas, las claves viven en los propios huecos.
- [Lista simplemente enlazada](learn:linear/singly-linked-list): la forma clásica de construir cada cadena.
- [Coste amortizado: el arreglo dinámico](learn:foundations/dynamic-array): por qué duplicar al redimensionar sigue siendo barato.
- [Caché LRU](learn:linear/lru-cache): un mapa hash junto a una lista de uso.
