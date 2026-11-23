# C10: fin de ruta fuera de ciudad

PC de Sergio. En `Convoy::advance` (0x502110), si el destino no identifica
una ciudad y el convoy llega al último punto, 0x5028CC libera primero el
array de puntos y después el descriptor de una ruta propia; después borra el
puntero del barco. Las rutas compartidas se desasocian sin liberar sus datos.
El anfitrión registra las liberaciones y solo confirma descriptores y puntos
tras completar el tick; valida las direcciones sintéticas recibidas.

`x86_c10_convoy_route_free.py` ejecuta cuatro escenarios sintéticos del EXE
inglés original: dos posiciones, ruta propia y compartida. Intercepta la
búsqueda de ciudad para devolver «fuera de ciudad» y observa `operator delete`;
no demuestra paridad del buscador ni de todos los eventos marítimos. El test
Rust compara imágenes completas de barco y convoy, prueba dos líderes en el
mismo tick, revierte la liberación cuando el segundo convoy falla y guarda y
recarga S05 con una partida local como contenedor.

La llegada fuera de ciudad es una alternativa normal de `cv_517260`; no se
trata como error en el anfitrión. Tras esta corrección ambas partidas
`TestAstiller` superan `cv_free` y alcanzan el siguiente límite aún abierto,
`cv_obj_new` en tick 125727054. No se abrió el juego ni se afirma paridad de
las partidas de 30 días.
