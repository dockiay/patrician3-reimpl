# Barco suelto de misión pirata: desvinculación y convoy

`p3_core::ships::pirate_engage::prepare_single_ship_convoy(w, ship, output_initial)` prepara, sobre copias, la secuencia `0x00505B60` → `0x005062F0`. Devuelve gestor, flota, capitanes, comerciantes y ciudades, ID del convoy y efectos de interfaz (`ui_ships`); el llamante decide cómo publicar todo de forma atómica. No construye escena de asalto ni batalla.

`output_initial` es el valor previo de la variable de salida: el original lo copia a `convoy+8`, incluso si no es `0xFFFF`. El llamante debe suministrar el valor observado en su propio contexto; no se atribuye un valor universal a la pila del original.

El contraste directo `x86_c10_single_ship_convoy` ejecuta ambas funciones originales con `FPCW=0x027F`; compara las imágenes completas y la interfaz de 128 casos (32 flotas y cuatro valores de salida). Los callbacks de interfaz se interceptan y se recogen; no se interceptan las dos funciones ni las operaciones de flota a las que llaman. Otra prueba comprueba que pool lleno, barco ausente y lista cíclica se rechazan sin publicar cambios.

Límites: no se amplía el pool de convoyes; no hay persistencia S02/S05 autónoma, que debe completar la transacción llamante. El test usa estados sintéticos derivados de un fixture de encuentros navales; no constituye una ejecución completa de la misión ni una prueba en partida.

Para la misión `0x4EAFE4`, `prepare_pirate_mission_single_ship_convoy` inserta entre las dos llamadas la bandera `ship+0x3C |= 0x20`, el estado `ship+0x134 = 0x14` y la segunda eliminación del orden X; después asigna `convoy+0x14 = 0x12`. El oráculo `x86_c10_mission_single_ship.py` parte de `0x4EAFE4` y compara la instantánea anterior a la escena (`0x4EBB72`) en 128 casos, inyectando en la pila cuatro valores previos de la variable de salida. La presentación y la escena se interceptan, no se prueba la escena completa. Una prueba adicional verifica el rollback cuando se agota el pool tras las modificaciones intermedias.

Si el barco no figura en la lista marítima, la API rechaza el estado inconsistente. El original `0x505B60` devuelve éxito y modifica el enlace terminal incluso en ese caso; esta salvaguarda no reproduce esa ruta corrupta.
