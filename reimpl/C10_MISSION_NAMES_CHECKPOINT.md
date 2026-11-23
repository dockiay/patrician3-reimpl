# C10: propagación del generador de nombres

Equipo: PC de Sergio. Base de integración `db19fbd7`.

El gestor naval y la fábrica representan el mismo contador de nombres (+0xEE)
y su paso (+0xF0). Al publicar la navegación, ahora los cambios de estos dos
campos se propagan a la fábrica. Antes una creación naval podía dejar ambas
vistas distintas y la tarea de guion 0x1B terminaba con «generador de apellidos
ausente», aunque el problema era que se descartaba su contexto completo.

- `cargo test -p p3-sim --test c10_absorption_clock`: 1/1, con partida,
  CPR y EXE ingleses locales. La prueba de propagación fuerza una transición
  calculada con el generador nativo; los primeros 1000 ticks de esa partida
  no crean un barco y no acreditan por sí solos la ruta de creación.
- `cargo test -p p3-mvp --test c10_corpus` con filtro
  `fps-cache/save/Ein/AUTO.pat`: dos ejecuciones deterministas de 30 días,
  guardado repetido, recarga y un día adicional; 1/1. El fallo anterior se
  producía en el tick 125736723; las ejecuciones llegan al 125737813 y,
  tras recarga, al 125738069. Informe local ignorado:
  `coordination-runtime/c10-mission-names-corpus.json`.

El contraste anterior acredita ejecución nativa y persistencia para esta
partida y configuración; no compara los 30 días completos contra x86 ni
valida el comportamiento en partida. Siguiente paso: integrar los commits
en la rama de C10, repetir la prueba del coordinador y ampliar el corpus y
los contrastes con el original antes de cerrar C10.
