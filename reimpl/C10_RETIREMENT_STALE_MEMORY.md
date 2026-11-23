# C10: bytes liberados durante el ciclo decenal

La guarda de producción que rechaza crecer el planificador al procesar
jubilaciones de `0x03` sigue siendo necesaria. No hay información suficiente
en S05 para reconstruir el estado, los permisos ni los alias del heap del
proceso original.

En `0x4D85C0`, `EDI` apunta al registro activo del array anterior. El ciclo
`0x4DDA40` ejecuta la jubilación original, cuyo alta puede ampliar el array y
liberar su dirección. Tras volver, `0x4D8668` lee el dword de `[EDI]`, y
`0x4D8671` escribe en ese mismo dword la suma modular de `0xA00`. La rutina
`0x4D8DD0` posterior vuelve a leer desde el planificador la base del array
vivo. La dirección obsoleta es `old_base + 24 * old_head`, no necesariamente
el comienzo de la reserva.

El oráculo `x86_c10_retirement_stale_word.py` ejecuta el despachador, ciclo,
jubilación y crecimiento originales. En 18 escenarios combina las cabezas
0, 5 y 127 con seis imágenes del dword liberado, incluidas sumas con
desbordamiento. Un hook observa todos los accesos a la página antigua desde
su liberación: en esos escenarios son exclusivamente la lectura y la escritura
del dword citado. Todos vuelven con dos ejecuciones de `0x03`; para una misma
cabeza, el estado vivo de cola y capitanes observado coincide entre imágenes.
También ejecuta los tres modelos de liberación con dos jubilaciones: conservar,
rellenar y desmapear; el último falla en la lectura 0x4D8668. Se adaptan la
consulta de comerciantes, purga de cartas y recuento de carga, y se emplean
modelos de heap controlados; no son mediciones del asignador de Windows ni una
prueba en partida.

Una imagen explícita del dword y de sus permisos en el instante de la lectura
permite reproducir este acceso aislado. Para reproducir sus efectos sobre una
sesión completa también haría falta saber si la dirección liberada se reutilizó,
a qué objeto o metadatos pertenece, y propagar allí la escritura. Una imagen
tomada al liberar tampoco garantiza el valor posterior si el bloque se reutiliza
antes de 0x4D8668. Por ello no se instala un puerto que suponga que el bloque
conserva sus bytes o carece de alias; el rechazo transaccional existente no se
confunde con un resultado universal del original.

Reproducción:

```powershell
D:/AI/Patrician3/tools/python/Scripts/python.exe reimpl/oracle/x86_c10_retirement_stale_word.py --binary D:/AI/Patrician3/games/iso-english/app/Patrician3.exe --check
```
