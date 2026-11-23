# Revisión de aceptación C10

PC de Sergio, 4/11/2026. Revisión independiente; C10 aún no aceptado.

## Presencia de punteros en la comparación

Se reprodujo un falso positivo cambiando únicamente el puntero de ruta del
barco0 en day-00 de0x20235510 aNULL: el comparador anterior declaraba igualdad.
Ahora conserva0 y convierte todas las direcciones no nulas a1. Las diferencias
de direcciones de proceso siguen admitidas; perder o inventar presencia no.

Se aplica el mismo criterio estricto de estado emitido a tarea1B+8/+C y cartas
29/47/63/83+8. Matiz: el cargador original reconstruye esos valores por otras
fuentes, sin consultar el puntero antiguo (4D82F9,4D830F,4D8337 y51AA87).
La presencia ahí es una exigencia conservadora de la comparación, no una prueba
de que el cargador distinga NULL/noNULL. El índice del guion, número y contenido
de argumentos, longitud y payload de cartas siguen comparándose completos.

Prueba sin recursos comerciales: `python -m unittest discover -s tests -p
test_c10_real_snapshots.py -v`. Cubre direcciones distintas válidas, NULL en ambos
sentidos, ambos punteros de tarea, cuatro clases de carta, payloads, tamaños y
registros truncados. No ejecuta juego ni repite el corpus.

## Condiciones para sellar la evidencia final

El tablero principal exige una partida real,30días y un resumen comparable; la
aceptación ampliada en curso exige los ocho bloques de32muestras y corpus9.
El informe original `c10-real-save-headless30.json` sí acredita32muestras,
125740230→125747910 (7680ticks), tres batallas IA en125741968/125744172/125746784
y315/207/123pasos de16. No hay decisión humana simulada. Las adaptaciones de
presentación están declaradas y el reloj no avanza durante la batalla.

Antes de aceptar deben quedar vinculados al resultado final:

- **Intervalo completo:** `compare_paths` compara los archivos presentes y
  exige el mismo conjunto, pero admite dos directorios con sólo day-00.
  El sello debe exigir day-00..day-30 más tick-000001, y sus ticks correctos.
- **Estado externo:** cotejar el RNG diario de ambos informes. La comparación
  del PAT no lo contiene. Registrar también el protocolo de batallas y su
  resultado; no presentar igualdad persistida como igualdad de todo el proceso.
- **Procedencia nativa:** el exportador imprime hashes de cada payload, pero
  no hashes de entrada/CPR/EXE/binario ni fuente compilada. Además admite un
  directorio existente y deja archivos de ejecuciones anteriores. Usar destino
  nuevo y manifiesto del ejecutable, entradas, parámetros y commit final; ligar
  cada hash del comparador a ese manifiesto y a una ejecución que terminó bien.
- **Caché x86:** su clave contiene EXE,partida,navegación y tabla urbana, pero no
  hash de todas las fuentes ni de recursos ya cargados. El informe registra una
  fuente de caché distinta de la fuente actual. Es aceptable sólo tras revisar
  las diferencias anteriores al checkpoint; no basta con que los hashes de
  navegación coincidan. Una carga fresca o un manifiesto de todas las fuentes y
  recursos cacheados elimina la ambigüedad. No se ha demostrado corrupción en
  el checkpoint actual; es una limitación de procedencia.
- **Corpus final:** el informe `c10-corpus-letters-integrated.json` ya muestra
  nueve partidas aprobadas y filtro vacío. Es anterior a las correcciones de
  rotación/expedición, por lo que no sustituye la ejecución de la versión final.
  Registra PAT,EXE,CPR y ejecutable de test, pero no el hash del PST de entrada
  opcional. Añadirlo al manifiesto permite reproducir también las estadísticas.

La prueba de recarga del corpus demuestra dos continuaciones recargadas iguales,
no equivalencia con31días continuos: lo declara `reload_limit`. El cargador
original recalcula precios y pesos. No exigir continuidad idéntica sin aplicar
el mismo cargador a ambas ramas, ni atribuir al corpus esa prueba adicional.

No aparecen otras omisiones demostradas de lógica en el protocolo de batalla
revisado. La aceptación quedará acotada a simulación sin pantalla, perfiles y
partidas ensayados; no a interfaz, multijugador o juego humano completo.

## Herramienta de sellado estricto

`reimpl/oracle/accept_c10_real_run.py` implementa las comprobaciones de intervalo,
archivos,perfil,RNG y resumen anteriores. Exige exactamente32nombres, ambas
marcas de ejecución completa,30días y el tick guardado realmente en cada
`game_world`; no confía sólo en el tick del JSON. Cruza hashes de los archivos,
los ocho bloques completos y todos los escalares y vectores del resumen.
Comprueba además los hashes de PAT,CPR,EXE y ejecutable nativo, las cinco fuentes
compiladas declaradas y los recursos abiertos por el oráculo frente al CPR.

Ejemplo desde el worktree de la versión compilada:

```powershell
python reimpl/oracle/accept_c10_real_run.py --original-report ORIGINAL.json --native-manifest NATIVO/manifest.json --original-dir ORIGINAL --native-dir NATIVO --save AUTO.pat --archive p2arch0_eng.cpr --binary Patrician3.exe --native-executable c10_snapshots.exe --source-root . --output ACEPTACION.json
```

El informe devuelve `accepted:false` ante cualquier diferencia; lista las
muestras con RNG,resumen o bloques distintos. `accepted:true` acredita estos
criterios técnicos del recorrido; no cambia el tablero ni sustituye la revisión
manual del origen del caché. Conserva el hash del caché y esa limitación explícita.

Pruebas:22unittest (14del sello y8del comparador), con casos sintéticos que
rechazan ejecuciones incompletas, duplicados, restos de otra ejecución, ticks
falsos, archivos alterados, RNG o resumen distintos, NULL perdido y procedencia
incorrecta. Una comprobación ligera de procedencia del informe original real
cotejó57recursos con el CPR; no ejecutó de nuevo la simulación y no es un sello
C10. El primer día real sigue dando8/8bloques iguales con presencia preservada.

El sello verifica también los archivos reales `x86_c10_real_save_clock.py` y
`c10_real_battle_driver.py` de `--source-root`, normalizando CRLF aLF como hacen
sus informes. Una fuente o driver editado impide aceptar aunque el JSON conserve
su hash anterior. Tres regresiones adicionales cubren CRLF y ambas alteraciones.
Esto no sustituye la revisión separada de la fuente histórica del caché.
