Empieza por `bookings` y une `flights` para tener el número de vuelo y los dos códigos de aeropuerto. ¿Cómo conviertes cada código en una ciudad?
---
Cada vuelo necesita **dos** ciudades, de dos filas distintas de `airports`. Une `airports` dos veces con dos alias: `JOIN airports src ON src.code = f.from_code` y `JOIN airports dst ON dst.code = f.to_code`.
---
Selecciona `b.passenger, f.flight_no, src.city AS from_city, dst.city AS to_city`. `departs` se escribe `AAAA-MM-DD HH:MM`, así que como texto se ordena por tiempo: termina con `ORDER BY b.passenger, f.departs, b.id`.
