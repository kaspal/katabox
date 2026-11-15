## Idea

Cada reserva se convierte en una fila tras tres búsquedas:

1. **Unir** la reserva con su vuelo, para tener el número de vuelo, los dos códigos de aeropuerto y la hora de salida.
2. **Unir `airports` dos veces**, una por cada extremo del vuelo, para convertir ambos códigos en ciudades.
3. **Ordenar** por pasajero y luego por hora de salida.

## La consulta

```sql
SELECT b.passenger, f.flight_no, src.city AS from_city, dst.city AS to_city
FROM bookings b
JOIN flights f ON f.id = b.flight_id
JOIN airports src ON src.code = f.from_code
JOIN airports dst ON dst.code = f.to_code
ORDER BY b.passenger, f.departs, b.id;
```

## Detalles que importan

- **Dos copias de `airports`.** Una fila unida de `airports` es un solo aeropuerto, así que no puede ser los dos extremos de un vuelo. Si se une una sola vez por `from_code` y se usa su ciudad en las dos columnas, cada tramo aterriza donde despegó; cada extremo necesita su propio alias (`src` y `dst`) y su propia condición `ON`.
- **Los alias nombran las columnas.** Las dos copias tienen una columna llamada `city`, así que hacen falta `AS from_city` y `AS to_city` para dar al resultado sus nombres de columna.
- **Las horas se ordenan como texto.** `AAAA-MM-DD HH:MM` con ceros a la izquierda deja antes las horas más tempranas al comparar cadenas, así que `ORDER BY f.departs` es el orden de salida sin funciones de fecha. `departs` no está en el resultado, y `ORDER BY` lo permite.
- **Un orden completo.** Las horas de salida son distintas, pero un pasajero puede reservar el mismo vuelo dos veces; el `id` de la reserva deshace ese último empate.
- **Bastan uniones internas.** Cada reserva tiene su vuelo y cada vuelo sus dos aeropuertos, así que no se pierde ninguna fila. Los vuelos que nadie reservó no dan filas, como muestra el ejemplo 2.

## Coste

Cada reserva necesita tres búsquedas por clave primaria (su vuelo y luego dos aeropuertos), así que las uniones cuestan más o menos un paso por reserva; ordenar el resultado cuesta `O(b log b)` para `b` reservas.
