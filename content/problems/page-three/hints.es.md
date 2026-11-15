Primero construye la lista completa que mostraría la web: quita los anuncios ocultos y ordena el resto. Después piensa cuántos anuncios van antes de la página 3.
---
`WHERE hidden = 0 ORDER BY rating DESC, title, id` da los anuncios visibles en el orden de las páginas. Entre las páginas 1 y 2 hay 20 anuncios, así que la página 3 empieza tras saltar 20, no 30. SQL puede saltar filas y limitar cuántas devuelve después de ordenar.
---
Añade `LIMIT 10 OFFSET 20` tras el `ORDER BY`: salta las 20 primeras filas y devuelve como mucho 10. Si hay menos de 21 anuncios visibles, simplemente no devuelve filas.
