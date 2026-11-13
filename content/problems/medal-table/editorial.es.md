## Enfoque: ordenar por una clave compuesta

Empareja cada país con sus medallas y ordena con una comparación que mire las partes en orden:

1. más oros primero;
2. si empatan, más platas primero;
3. si empatan, más bronces primero;
4. si empatan, el nombre en orden alfabético.

Con una ordenación por clave, la clave `(−oros, −platas, −bronces, nombre)` hace los cuatro pasos a la vez: las tuplas se comparan elemento a elemento, y negar los recuentos convierte "más primero" en orden ascendente.

Los nombres son solo letras minúsculas, así que la comparación carácter a carácter es el orden alfabético (sin reglas de idioma).

**Complejidad:** O(n log n) comparaciones.
