# Guía de uso de KataBox

KataBox es una aplicación de escritorio gratuita y sin conexión para practicar
programación. Resuelves problemas en Python, JavaScript/TypeScript, PHP o Rust (y SQL en
los problemas SQL); tu código se ejecuta en tu propio equipo, dentro de un sandbox, y se
evalúa con pruebas públicas y ocultas. No se envía nada a ningún sitio: lo único que se
descarga son los entornos de ejecución de los lenguajes que elijas instalar.

Qué incluye:
- **209 problemas:** 150 de función, 35 de programa (stdin/stdout) y 24 SQL; 68 fáciles,
  90 de dificultad media y 51 difíciles, agrupados en 27 patrones.
- **Aprender:** 118 temas animados en 14 materias, más 3 rutas de aprendizaje.

Los nombres de los botones son los de la interfaz en español; la guía en inglés usa los
ingleses.

## Primeros pasos

### Instalación
- KataBox funciona en **Windows 10/11 (x64)**. Ejecuta el instalador
  `KataBox_<versión>_x64-setup.exe`: instala solo para tu usuario, sin permisos de
  administrador.
- El instalador aún no está firmado, así que Windows SmartScreen puede avisar la primera
  vez: elige **Más información** y luego **Ejecutar de todas formas**.

### La bienvenida
En el primer inicio KataBox te guía en cuatro pasos cortos. **Omitir la bienvenida**
(arriba a la derecha) está disponible en todos ellos.
1. **Idioma de la interfaz:** English o Español, arriba en la pantalla de bienvenida.
2. **Elige los lenguajes que quieres instalar:** marca Python, JavaScript / TypeScript,
   PHP o Rust y pulsa **Instalar selección** (o **Ahora no**). Cada entorno se descarga
   una sola vez de su sitio oficial, se comprueba con una suma de verificación fijada y
   se guarda dentro de la carpeta de KataBox. Antes de instalar se muestran el tamaño de
   la descarga y el espacio en disco.
3. **¿Cuánto has practicado?**
   - **Empiezo de cero:** empiezas por los fundamentos.
   - **Ya he practicado:** una **Prueba de nivel** corta de tres problemas (dos fáciles y
     uno medio). Resuelve cada uno si puedes o pulsa **Siguiente problema**; **Terminar
     la prueba** fija el patrón y la dificultad de partida.
4. **El recorrido:** cinco indicaciones en el espacio de trabajo (Ejecutar, Enviar,
   Pistas, Resultados y depurador, Aprender). Usa **Siguiente** / **Atrás**, o → / ←;
   Esc o **Saltar el recorrido** lo terminan.

Puedes repetirla desde **Ajustes → Práctica → Mostrar de nuevo la bienvenida**.

### Cómo moverte
La barra de la izquierda abre **Inicio**, **Biblioteca**, **Aprender**, **Entrevista**,
**Progreso** y **Ajustes**. La **Paleta de comandos** (Ctrl+K, o el botón de la barra
superior) salta a cualquier página, problema o tema de Aprender, y cambia el tema y el
idioma.

- **Inicio:** **Continuar** (tu último borrador), **Resueltos**, el **Problema del día**,
  tu **Repaso** y la tarjeta **Repasar un tema**.
- **Biblioteca:** busca (pulsa `/`), filtra por dificultad, estado, patrón, etiqueta y
  tipo de problema (Función, Programa, SQL) y ordena por cualquier columna. ↑ / ↓ para
  moverte, Intro para abrir. La pestaña **Listas** guarda tus propias listas.
- **Progreso**, en cuatro pestañas:
  - **Resumen:** problemas resueltos, el mapa de **Actividad** de 12 meses, **Resueltos
    por dificultad**, la **Cola de repaso** y **Resueltos por lenguaje**;
  - **Por patrón:** el mapa de **Patrones**;
  - **Por tema:** resueltos por etiqueta (**Mostrar** todas) y los **Temas de Aprender
    vistos** de cada materia;
  - **Historial:** las entrevistas simuladas anteriores.

  La próxima vez se abre la última pestaña que usaste.

## Problemas y espacio de trabajo

### Tres tipos de problema
- **Función** (el habitual): escribe la función del código inicial; el juez la llama con
  los argumentos de cada prueba y compara el valor devuelto.
- **Programa** (etiqueta **Programa · stdin/stdout**): tu archivo es el programa
  completo. Lee toda la entrada estándar y escribe la respuesta en la salida estándar.
  La salida se compara token a token (algunos problemas comparan líneas o admiten un
  margen en los decimales). Escribe los mensajes de depuración en **stderr**, porque
  stdout es la respuesta. Los tiempos incluyen el arranque de cada ejecución.
- **SQL** (etiqueta **SQL · una consulta**): escribe un único `SELECT` (o
  `WITH … SELECT`) sobre las tablas del problema, que ves en la pestaña **Tablas** con
  columnas, claves y filas de ejemplo. Las consultas se ejecutan en modo de solo lectura
  sobre SQLite con el Python incluido, así que **Python debe estar instalado**. El
  problema indica si el orden de las filas importa. Los problemas SQL solo admiten SQL,
  y no tienen pruebas personalizadas, depurador, perfilador ni reductor de entradas.

### La distribución
- **Izquierda, Problema:** pestañas **Descripción**, **Pistas**, **Solución**,
  **Envíos**, **Notas**, **Historial** (y **Tablas** en SQL), con los límites de tiempo y
  memoria. **Míralo animado** enlaza con los temas de Aprender que usan el problema.
- **Arriba a la derecha, el editor:** elige el **Lenguaje** en la cabecera. Los
  borradores se guardan solos (**Guardando… / Guardado**), por problema y por lenguaje.
  Con TypeScript, **Revisar el código como** alterna entre **Revisión de JavaScript**
  (sin anotaciones de tipos) y **Revisión de TypeScript** (estricta); Ejecutar y Enviar
  funcionan igual en ambos casos.
- **Abajo a la derecha, Pruebas y resultados:** **Casos de prueba**, **Resultado** y
  **Depurar**.
- Muestra u oculta paneles con **Mostrar u ocultar el problema** / **Mostrar u ocultar
  pruebas y resultados**, o arrastra los separadores. El **Modo concentración**
  (Ctrl+Mayús+F) lo oculta todo salvo el editor y los resultados.
- La ayuda al escribir viene de servidores de lenguaje: basedpyright (Python),
  typescript-language-server (JavaScript/TypeScript) y rust-analyzer (Rust). PHP tiene
  autocompletado básico de palabras clave y funciones comunes; SQL completa las tablas y
  columnas del problema. La línea de estado bajo el editor muestra el estado del
  servidor, los errores y advertencias, la posición del cursor y un botón **Formatear**.

### Ejecutar o Enviar
- **Ejecutar** (Ctrl+'): evalúa las pruebas **públicas** y tus pruebas
  **personalizadas**. No registra un envío.
- **Enviar** (Ctrl+Intro): evalúa **todas** las pruebas, también las ocultas, y guarda el
  resultado en **Envíos**. Solo Enviar cuenta para resolver, para los repasos, el
  problema del día y la racha.
- Cada Ejecutar y Enviar guarda además una versión en **Historial**: selecciona dos para
  compararlas, o una para compararla con el editor, y **Restaurar** recupera una versión
  anterior (el código sustituido también se guarda como versión).
- **Resolver en…** (en un problema resuelto) muestra tu solución aceptada en otro
  lenguaje, en solo lectura, junto al editor, para que la traduzcas.

### Pruebas
- Las pruebas **públicas** (y los ejemplos del enunciado) muestran su entrada y la salida
  esperada. Las **ocultas** no, salvo el **Primer fallo** de un envío, que muestra su
  entrada.
- **Añadir prueba personalizada:** escribe tu propia entrada (valores JSON por
  parámetro, o un texto de stdin en un programa). La salida esperada la calcula la
  solución de referencia. Las entradas se revisan mientras escribes; con Python
  instalado también se comprueban contra las restricciones del problema. Las pruebas
  personalizadas no se guardan al salir del problema.
- En una prueba que falla: **Entrada**, **Esperado** y **Tu salida**, con la primera
  diferencia marcada, más lo que se imprimió y, en los programas, la **Salida de errores
  (stderr)**.

### Veredictos
**Aceptado**, **Respuesta incorrecta**, **Tiempo límite excedido**, **Memoria límite
excedida**, **Salida límite excedida**, **Error en ejecución**, **Error de compilación** y
**Error interno** (un fallo del juez, no tuyo). Cada uno viene con un mensaje corto que
lo explica.

### Límites de tiempo por lenguaje
El límite de tiempo de un problema está pensado para Rust; los demás lenguajes reciben
más tiempo. El espacio de trabajo muestra el del lenguaje elegido, p. ej. **Tiempo límite
(Python)**.
- Rust ×1, TypeScript ×2, PHP ×5, Python ×10, más un margen de arranque por lenguaje.
- KataBox además mide tu equipo una vez (**Velocidad del juez**, ver Ajustes) y ajusta
  cada límite entre ×0,5 y ×3; entonces el espacio de trabajo indica **ajustado ×N para
  este equipo**.

## Pistas, solución, notas y repasos
- **Pistas:** tres niveles, de un empujoncito a la idea completa. Se muestran en orden
  (**Mostrar la pista 1**, luego la 2 y la 3).
- **Solución:** bloqueada hasta que resuelves el problema, o tras 3 envíos fallidos.
- **Notas:** una nota en Markdown por problema (**Editar** / **Vista previa**), guardada
  automáticamente.
- **Repasos espaciados:** tu primer Aceptado en un problema programa un repaso a los 3
  días.
  - Empieza uno desde Inicio (**Repasar**) o Progreso (**Repasar ahora**). En modo repaso
    cada lenguaje empieza desde el código inicial, y tu código anterior (Historial,
    Envíos) queda oculto hasta que lo superes.
  - Superarlo lleva el repaso a 7 días y después a 21; superar el de 21 días significa
    que dominas el problema y sale de la cola. Si fallas, vuelve a 3 días.
  - **Posponer 1 día**, **Quitar del repaso**, o vuelve a añadir un problema con
    **Repasar en 3 días** desde el espacio de trabajo.
- **Problema del día:** uno nuevo cada día (según tu fecha local), con preferencia por
  tu patrón más flojo y nunca uno resuelto mientras queden pendientes. Resolverlo con
  Enviar ese mismo día alarga tu **Racha diaria**.
- **Siguiente problema:** tras un envío Aceptado, **Siguiente** propone el siguiente
  problema sin resolver del mismo patrón y después de los patrones que siguen. Los
  problemas SQL no aparecen en el problema del día ni en «Siguiente» hasta que hayas
  probado uno.
- **Listas:** **Añadir a una lista** desde una fila de la Biblioteca o la cabecera del
  espacio de trabajo; gestiónalas en **Biblioteca → Listas** (**Nueva lista**,
  **Renombrar**, **Eliminar**; arrastra las filas o usa Alt+↑ / Alt+↓ para ordenarlas).
- Activa o desactiva las tarjetas de Inicio en **Ajustes → Práctica** (la tarjeta
  **Repasar un tema** está en **Ajustes → Aprender**).

## Entrevistas simuladas
**Entrevista → Empezar una entrevista simulada:**
- elige la **Duración** (30, 45 o 60 min), la **Dificultad** de los dos problemas (p. ej.
  Fácil · Media) y los **Lenguajes** permitidos, y pulsa **Empezar**;
- recibes dos problemas sin resolver que nunca has abierto (nunca SQL);
- un temporizador aparece en la barra superior y en el espacio de trabajo, también en
  modo concentración, y sigue corriendo aunque cierres la app;
- las pistas, la solución, los envíos anteriores, el Historial y **Resolver en…** quedan
  **Bloqueado durante la entrevista simulada**. Ejecutar, Enviar, Notas y el perfilador
  de complejidad siguen disponibles;
- termina al acabarse el tiempo o con **Terminar la entrevista**, y muestra el
  **Resultado** (resueltos, aciertos a la primera, tiempo usado, y tiempo y envíos por
  problema). Las entrevistas anteriores están en Progreso.

## Katas
**Kata** (en la barra de navegación) vuelve a resolver problemas que ya resolviste, de
memoria y contra el reloj:
- **Kata del día:** tres problemas resueltos elegidos para hoy, con preferencia por los
  que llevas tiempo sin tocar. Unos 15 minutos; el reloj muestra el tiempo de más pero
  nunca te detiene.
- **Elige un conjunto:** hasta cinco problemas resueltos de un patrón o de una de tus
  listas, los más oxidados primero, diez minutos cada uno.
- El editor empieza con el código inicial. Las pistas, la solución explicada, tus envíos,
  el historial y las notas quedan **bloqueados durante la kata** hasta que termina.
- Una kata nunca cambia tus envíos, el estado de los problemas, los repasos ni el problema
  del día. Sus resultados están en **Progreso → Katas**: tus récords (mejor tiempo, menos
  envíos), la racha de katas y las katas recientes.
- **Oxidados** (tarjeta de Inicio, y en Progreso → Katas) muestra problemas resueltos que
  quizá hayas olvidado: la puntuación crece con el tiempo desde la última vez que lo
  resolviste y con las katas fallidas en él. Desactiva la tarjeta en Ajustes → Práctica.

## Nivel estimado y planes de estudio
- **Nivel estimado:** Progreso → Resumen muestra un nivel general, y Por patrón uno por
  patrón con una pequeña línea de evolución. Se **estima** a partir de los problemas que
  resolviste (su dificultad, tus envíos y tu tiempo) y orienta lo que sugieren el problema
  del día y «Siguiente problema»: un patrón más flojo aparece más a menudo, y se saltan
  los problemas fáciles que ya superaste. Es una guía, no una nota.
- **Plan de estudio:** Progreso → Plan → **Crear el plan** a partir de un objetivo (todos
  los patrones o los que elijas), un número de semanas y el tiempo que tienes al día.
  Primero van los temas de Aprender de cada patrón y luego sus problemas sin resolver; el
  último día de cada semana tiene una kata y tus repasos. Marca las tareas como
  **Hecho**, **Omítelas** o pásalas a **Más tarde** (el resto del plan se mueve con
  ellas). Los problemas resueltos y los temas vistos se marcan solos. Las tareas de hoy
  están en la tarjeta **Plan de estudio** de Inicio (desactívala en Ajustes → Práctica).
  **Exportar al calendario…** guarda el plan como archivo `.ics` para cualquier
  aplicación de calendario.

## Depurador y visualizador
- **Lenguajes:** Python y JavaScript/TypeScript, en problemas de función y de programa.
  Aún no PHP, Rust ni SQL.
- **Empezar:** **Depurar este caso** en cualquier prueba (o en un fallo), o F5 para
  depurar la primera prueba pública. Pon puntos de interrupción haciendo clic justo a la
  derecha de un número de línea, o con F9 en la línea del cursor.
- **Reproducción, no en vivo:** cada paso vuelve a ejecutar la prueba en el sandbox hasta
  ese punto, así que también puedes dar **Paso atrás**. Un programa que se comporta
  distinto en cada ejecución (hora, azar) no se puede reproducir. Las ejecuciones muy
  largas dejan de grabar tras cierto número de pasos; esos pasos se pueden recorrer
  igualmente. El editor es de solo lectura mientras depuras.
- **Barra:** **Continuar** (F5), **Paso a paso por encima** (F10), **Paso a paso por
  instrucciones** (F11), **Paso a paso para salir** (Mayús+F11), **Paso atrás**
  (Mayús+F10), **Reiniciar** (Ctrl+Mayús+F5), **Detener la depuración** (Mayús+F5).
- **Paneles:** **Variables** (Locales / Globales, desplegables), **Inspección**, **Pila
  de llamadas** y **Consola de depuración** (evalúa en el marco seleccionado; nunca
  cambia el programa).
- **Pestaña Visualizar:** dibuja las variables que elijas en cada paso: arreglos con
  variables puntero, cuadrículas, listas enlazadas, árboles, conjuntos, mapas y grafos,
  marcando los cambios desde el paso anterior. **Dibujar … como** alterna entre
  **Automático**, **Grafo** y **Montículo (árbol)**; **Abrir en grande** muestra una
  vista ampliable. Los valores grandes se recortan para que cada paso siga siendo rápido.

## Perfilador de complejidad
En los problemas que lo admiten, un Ejecutar o Enviar Aceptado muestra el botón
**Complejidad** en la pestaña Resultado.
- Ejecuta tu solución con entradas de tamaño creciente (3 veces cada una, unos 15
  segundos como mucho) y ajusta las curvas de crecimiento más comunes.
- El resultado siempre es **Estimada: O(…)** (a veces «O(a) o O(b)»), con una nota de
  confianza, una gráfica log-log frente a la solución de referencia y una tabla de
  tiempos. Los tiempos varían entre ejecuciones y equipos, así que es una estimación, no
  una demostración.
- En los programas se descuenta el tiempo de arranque; si lo que queda es demasiado
  pequeño, lo dice en lugar de adivinar.

## Buscar una entrada más pequeña que falle
En una prueba que falla y cuya entrada puedes ver (pública, personalizada o el primer
fallo), tras Respuesta incorrecta, Error en ejecución, Tiempo límite o Memoria límite
excedidos:
- **Buscar una entrada más pequeña que falle** reduce la entrada paso a paso mientras tu
  código siga fallando (como mucho 60 ejecuciones en el sandbox o 30 s; **Detener** en
  cualquier momento).
- El resultado ofrece **Depurar este caso** y **Añadir a mis pruebas**.
- Solo en problemas de función: la entrada de un programa tiene recuentos y líneas de
  estructura que no se pueden recortar sin romperla.

## Aprender
**Aprender** enseña algoritmos y estructuras de datos con animaciones.
- **Inicio:** busca temas, **Continuar** donde lo dejaste, **Rutas de aprendizaje**
  (Empieza aquí, Lo esencial para entrevistas, Maestría en grafos) y las 14 **Materias**
  con tu progreso.
- **Un tema** tiene un escenario (la animación) y tres pestañas:
  - **Teoría**, con enlaces a temas relacionados;
  - **Código**: un ejemplo probado en Python o TypeScript; la línea resaltada es la que
    corresponde al paso actual, con sus **Variables**;
  - **Práctica**: preguntas de **Predice el siguiente paso** y después **Problemas para
    practicar**, que se abren en el espacio de trabajo.
- **El reproductor:** **Reproducir** / **Pausa**, **Paso atrás**, **Paso adelante**,
  **Reiniciar**, una **Línea de tiempo** para desplazarte y **Velocidad** (0,5×, 1×, 2×,
  4×). Cambia la **Entrada** con un ejemplo predefinido o con tus propios valores
  (**Aplicar**).
- **Comparar con…** (en temas que aceptan la misma entrada, como los de ordenación y
  búsqueda): dos algoritmos lado a lado con una misma entrada.
- Un tema cuenta como **visto** cuando llegas a su último paso reproduciendo o avanzando
  paso a paso.
- Enlaces con la práctica: los enlaces **Aprender:** en el filtro de patrón de la
  Biblioteca y en el mapa de patrones de Progreso, **Míralo animado** en el espacio de
  trabajo y la tarjeta **Repasar un tema** en Inicio.

## Exportar e importar
**Ajustes → Datos:**
- **Exportar…** guarda tu progreso en un archivo (`.kbx.json`, formato
  `katabox-export/v2`; los archivos v1 se siguen importando): estados de problemas, envíos, instantáneas del historial, notas,
  borradores, pistas desbloqueadas, problemas abiertos, repasos, problemas del día,
  listas, entrevistas simuladas, ajustes y tu progreso en Aprender (temas vistos y
  posición de Continuar).
- No se exportan: el factor de velocidad de este equipo, su identificador de instalación
  ni su carpeta del sandbox.
- **Importar…** muestra una vista previa (qué contiene el archivo y los conflictos) antes
  de pulsar **Importar**. Combina por problema y conserva el registro más reciente;
  importar dos veces el mismo archivo no cambia nada. Un archivo no válido se rechaza
  entero, y nada de un archivo se ejecuta.

## Ajustes
Los ajustes tienen una sección por tema, a la izquierda (o en pestañas arriba, si la
ventana es estrecha). **Buscar en los ajustes** encuentra un ajuste por su nombre.

- **Apariencia:**
  - **Tema** (Sistema, Claro, Oscuro), **Idioma** (English, Español), **Contraste**
    (Normal, Alto), **Movimiento** (Seguir al sistema, Reducir);
  - **Restablecer el tamaño y la posición de la ventana**. KataBox se vuelve a abrir
    donde lo dejaste, con el mismo tamaño y maximizado si lo estaba. Si esa pantalla ya
    no está, se abre centrado en la pantalla principal.
- **Editor:** **Atajos de teclado** (Predeterminado, VS Code, Vim, Emacs), **Ligaduras
  de la fuente**, **Tamaño de letra** (12-20 px), **Ancho de tabulación** (El del
  lenguaje, 2, 4 u 8 espacios), **Tema de colores** (KataBox, Tinta intensa, Papel,
  Anochecer), con **Vista previa** en directo.
- **Práctica:** **Problema del día en Inicio**, **Recordatorios de repaso en Inicio**,
  **Mostrar de nuevo la bienvenida**.
- **Lenguajes y entornos de ejecución:**
  - instala o **Desinstalar** entornos. Desinstalar conserva tus borradores, notas y
    envíos;
  - **Velocidad del juez:** el **Factor de velocidad** de este equipo, cuándo y con qué
    se midió, y **Volver a medir**.
- **Aprender:** **Tarjeta «Repasar un tema»** y la **Velocidad de las animaciones**.
- **Datos:** **Exportar…** / **Importar…** (ver arriba).
- **Acerca de:** versiones, la **Carpeta de datos** y la **Carpeta del sandbox**,
  **Avisos de terceros** y **Copiar diagnóstico…**.

## Atajos de teclado
Un diálogo **Atajos de teclado** los muestra dentro de la app: pulsa F1, elígelo en la
paleta de comandos o usa el botón del teclado en la barra superior.

| Dónde | Teclas | Acción |
| --- | --- | --- |
| En cualquier sitio | Ctrl+K | Paleta de comandos |
| Biblioteca | `/` | Buscar problemas |
| Biblioteca | ↑ / ↓, RePág / AvPág, Inicio / Fin, Intro | Recorrer la lista, abrir un problema |
| Biblioteca → Listas | Alt+↑ / Alt+↓ | Mover una fila de la lista |
| Espacio de trabajo | Ctrl+' | Ejecutar |
| Espacio de trabajo | Ctrl+Intro | Enviar |
| Espacio de trabajo | Ctrl+Mayús+F | Activar o salir del modo concentración |
| Espacio de trabajo | Esc (fuera del editor) | Salir del modo concentración |
| Editor | Mayús+Alt+F | Formatear documento (con servidor de lenguaje) |
| Editor | F12 | Ir a la definición (con servidor de lenguaje) |
| Editor | Ctrl+F, Ctrl+Z, Ctrl+Y | Buscar, deshacer, rehacer (Vim y Emacs añaden sus propias teclas) |
| Depurador | F9 | Poner o quitar un punto de interrupción en la línea del cursor |
| Depurador | F5 | Empezar a depurar / Continuar |
| Depurador | F10 / Mayús+F10 | Paso a paso por encima / Paso atrás |
| Depurador | F11 / Mayús+F11 | Paso a paso por instrucciones / para salir |
| Depurador | Ctrl+Mayús+F5 / Mayús+F5 | Reiniciar / Detener la depuración |
| Reproductor de Aprender | Espacio | Reproducir o pausar |
| Reproductor de Aprender | ← / → | Paso atrás / adelante |
| Reproductor de Aprender | Inicio | Reiniciar |
| Mapa de actividad (Progreso) | Flechas, Inicio / Fin | Moverse entre días |
| Recorrido | → o Intro, ←, Esc | Siguiente, atrás, terminar |

## Dónde se guardan tus datos
- **Tu progreso** (user.db: borradores, envíos, notas, ajustes):
  `%APPDATA%\dev.katabox.app`.
- **Entornos de ejecución, sandbox y cachés:** `%LOCALAPPDATA%\dev.katabox.app`.
- **La propia app:** `%LOCALAPPDATA%\KataBox`.
- **Ajustes → Acerca de** muestra las carpetas exactas de datos y del sandbox.
- **Al desinstalar** (Configuración de Windows → Aplicaciones) se quita la app pero se
  conservan tus datos, salvo que marques la casilla para borrar los datos de la
  aplicación («Delete the application data») en el desinstalador. El desinstalador
  también elimina el perfil de Windows del sandbox y sus permisos sobre carpetas.
- **Para borrar los datos a mano** después de desinstalar, elimina las dos carpetas de
  arriba (pega `%APPDATA%\dev.katabox.app` y `%LOCALAPPDATA%\dev.katabox.app` en la
  barra de direcciones del Explorador de archivos). Exporta antes si quieres conservar tu
  progreso.

## Ayuda
- **Ajustes → Acerca de → Copiar diagnóstico…** copia versiones, opciones y códigos de
  errores recientes para informar de un error. Muestra exactamente lo que se copiará: sin
  código, rutas de archivos ni datos personales.
- Problemas conocidos y cómo evitarlos: [problemas conocidos](known-issues.es.md).
