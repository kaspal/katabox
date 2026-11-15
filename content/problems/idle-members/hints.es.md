Buscas socios para los que algo *no* existe: una visita en noviembre. ¿Qué construcción de SQL pregunta "¿no hay ninguna fila así?"?
---
`WHERE NOT EXISTS (SELECT 1 FROM visits v WHERE v.member_id = m.id AND ...)` conserva un socio solo cuando la subconsulta no encuentra nada. La subconsulta debe mirar solo las visitas de noviembre, no todas: una visita en octubre no dice nada de noviembre.
---
Las fechas escritas `AAAA-MM-DD` se comparan bien como texto, así que `v.day BETWEEN '2026-11-01' AND '2026-11-30'` es noviembre, con los dos extremos incluidos. Selecciona `m.id, m.name` y termina con `ORDER BY m.name, m.id`.
