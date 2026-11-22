# Modo aula

KataBox puede repartir tareas y recoger resultados sin servidor ni cuentas. Ambos viajan
como archivos que compartes como prefieras (correo, una carpeta compartida, una memoria
USB).

**No es a prueba de trampas.** KataBox funciona sin conexión en el ordenador del
estudiante, así que quien quiera puede cambiar sus datos. El archivo de resultados lleva
una cadena de hashes que muestra las ediciones *casuales* (un veredicto cambiado en un
editor de texto), pero cualquiera puede recalcularla. Úsalo para practicar y dar
retroalimentación, no para calificaciones decisivas.

## Para profesores: crear una tarea
1. Escribe `assignment.toml`:

   ```toml
   title = "Semana 3: dos punteros"
   due = "2026-12-01"                    # AAAA-MM-DD
   problems = ["budget-pair", "two-sum"] # problemas de tu paquete o ids integrados
   languages = ["python", "typescript"]  # python, typescript, php, rust
   time_limit_min = 90                   # opcional, de 5 a 600 minutos desde Empezar
   hints = false                         # opcional; false bloquea pistas y solución explicada
   ```

2. Escribe `pack.toml` (consulta "Paquetes de problemas" en la guía de uso) y crea el
   paquete. Tus propias carpetas de problemas son opcionales; una tarea de problemas
   integrados no necesita ninguna:

   ```sh
   kb-pack pack mi-problema-1 mi-problema-2 --manifest pack.toml --assignment assignment.toml --out semana3.kbpack
   ```

3. Envía `semana3.kbpack` a tus estudiantes.

## Para estudiantes
1. **Ajustes → Paquetes → Importar un paquete…**, revisa la vista previa (muestra la
   tarea), instálalo y reinicia.
2. Abre **Aula**. Lee las reglas y pulsa **Empezar** cuando estés listo. Mientras está en
   curso, sus problemas solo aceptan los lenguajes indicados y, con las pistas
   desactivadas, sus pistas, solución explicada, envíos anteriores e Historial están
   bloqueados. Solo cuentan los envíos hechos mientras está en curso. Termina cuando
   pulsas **Terminar ahora** o cuando se agota el tiempo.
3. **Exportar resultados**: escribe tu nombre, elige si incluir tu código, lee la lista
   de exactamente lo que contiene el archivo y guárdalo. Envía ese archivo a tu profesor.

## Para profesores: recoger resultados
1. Abre **Aula → Resultados de la clase → Importar archivos de resultados…** y elige
   todos los archivos que recibiste (puedes elegir varios a la vez). Cada archivo se
   comprueba: uno roto o ajeno se rechaza con el motivo, el mismo archivo dos veces se
   importa una vez, y uno editado tras exportarse se guarda y se marca **Editado tras
   exportarse**.
2. La tabla muestra una fila por estudiante y una columna por problema.
3. **Exportar CSV…** escribe una fila por estudiante y problema. Las celdas que podrían
   empezar una fórmula de hoja de cálculo llevan delante `'`.
