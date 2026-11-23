# C07: 9 guardados × 365 días en los dos modos de batalla (9/11/2026, nube Claude)

`c10 GUARDADO p2arch0_eng.cpr 365 4660 --celebration-state 0:0 --exe Patrician3.exe
--human-battles-auto --investigation-local --battle-pacing paused|clock`, main 953d3f8
más este cambio. Guardados en `games/*/save/Ein` (locales, fuera de Git).

| Guardado | `paused` | `clock` |
|---|---|---|
| fps-cache/AUTO | año completo, 31 batallas | para: expedición hanseática 0x23 sin imagen naval viva (tick 125790208) → «barco de muralla desincronizado» |
| fps-cache/TestAstiller | para: asedio 6, bytes de pila | **año completo**, 37 batallas destruidas con el reloj en marcha (2.297 ticks mundiales durante batallas) |
| visibility/TestAstiller | para: asedio 6, bytes de pila | **año completo**, igual que el anterior |
| lab-english/TEST25_ConvoyMar | para: subasta 0x0A (adjudicación) | para: bajas de convoy pendientes (`return_convoy.rs`) |
| visibility/ConvoyMar | para: subasta 0x0A | para: bajas de convoy pendientes |
| visibility/AUTO | para: subasta 0x0A | para: subasta 0x0A |
| visibility/ConvoyHecho | para: subasta 0x0A | para: subasta 0x0A |
| visibility/ConvoyAvista | para: Remove(ClearRoute(122)) | igual |
| visibility/ConvoyPrueba | para: Remove(ClearRoute(119)) | para: tarea 0x04, ranura de pila 0x004F5A20 |

Ninguna parada del modo `clock` ocurre dentro del código nuevo: todas son ramas ya
conocidas de otros trabajos (subasta y expedición: C10/Codex; bajas de convoy y
ClearRoute: hilo Rust; asedio y tarea 0x04: pendientes de bytes de pila). Todas las
batallas del modo `clock` terminaron como `destruida`; no hubo solapes ni fin de
intervalo con batalla viva.
