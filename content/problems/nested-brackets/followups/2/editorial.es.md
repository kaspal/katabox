## Idea

Un corchete de cierre sin nada que emparejar, o con un tipo distinto en la cima, parte el texto: ninguna parte equilibrada puede cruzarlo. Guarda una pila de posiciones con el último de esos muros en el fondo (`-1` al principio) y, encima, los corchetes que abren y siguen esperando. Cuando `text[i]` cierra el corchete de la posición de la cima, desapílalo: todo lo que hay después de la nueva cima, hasta `i`, está equilibrado, así que su longitud es `i − cima`; quédate con la mayor. Si no empareja, `i` pasa a ser el nuevo muro: vacía la pila y deja solo `i` en el fondo. Un corchete que abre y nunca se cierra se queda en la pila y hace de muro para las partes que vienen después.

## Complejidad

O(n) en tiempo, porque cada posición se apila y desapila como mucho una vez, y O(n) de memoria extra para la pila.
