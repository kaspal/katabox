La cuenta empieza en `n` y baja en uno exactamente cuando una fusión une dos clubes **distintos**. Así que toda la pregunta es: ¿están ya `a` y `b` en el mismo club?
---
Dale a cada club un representante. Reetiquetar a todos los miembros de un club en cada fusión es correcto, pero un club grande se reetiqueta una y otra vez.
---
Union-find: cada jugador apunta a un padre, y siguiendo los padres se llega al representante del club. Fusiona apuntando un representante al otro; acorta los caminos al recorrerlos (compresión de caminos) y cuelga el club pequeño del grande (unión por tamaño).
