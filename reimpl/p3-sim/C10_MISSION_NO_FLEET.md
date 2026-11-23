# C10: misiones del alcalde sin dependencias navales

El despacho de la tarea `0x32` exigía una fábrica de flota y objetivos piratas antes de decidir qué fase se ejecutaba. Esto bloqueaba misiones urbanas que no consultan ninguno de esos datos. Las fases sin acceso naval se procesan ahora con ciudades y comerciantes; las fases que sí consultan barcos, capitanes u objetivos piratas mantienen sus dependencias explícitas. Para avisos de éxito y fracaso, cuando no hay recursos de expedición se utiliza el comerciante local del mundo, que ya identifica al jugador de la sesión.

## Contraste

Con el ejecutable inglés y una partida local, `c10_mission_no_fleet_x86` contrasta veinticinco escenarios en una ejecución x86 aislada del scheduler original `0x004D85C0`, sin flota ni destinos piratas:

- Población: inicio, espera, éxito y vencimiento humano.
- Carretera: inicio, espera de materiales y oferta sin aceptar.
- Fundación: espera de la ciudad asignada.
- Pasajero: inicio, éxito ya marcado, oferta válida o inválida, selección y espera con barco y capitán.
- Cancelación de misión.
- Ofertas sin asignar de fundación y piratas: destinos válidos e inválidos y umbral de selección de comerciante.
- Misión pirata asignada: éxito, vencimiento humano y aborto.

Se comparan las imágenes completas de las ciudades y comerciantes, la lista de tareas y el pool de cartas, además de los serializadores originales de tareas, cartas y comerciante. El S05 nativo se guarda y recarga, comprobando tareas, cartas, banderas de ciudad y estado del comerciante; también se comprueba que el segundo guardado es estable. Se verifican las solicitudes de diálogo de éxito y fracaso; las llamadas visuales del original se adaptan, no se abre el juego. El control x87 del oráculo es `0x027F`.

Ejecutar `cargo test -p p3-sim --test c10_mission_no_fleet_x86 -- --nocapture` con `P3_EXE`, `P3_ORACLE_PY` y `P3_SAVES` configuradas. Si faltan, la prueba se omite. Resultado local: 25/25 escenarios, más dos rechazos transaccionales al faltar los destinos.

## Límites

No se acredita con estos casos la aceptación y generación de misiones, el recorrido naval de un pasajero, la persecución y combate de la misión pirata, otras resoluciones de fundación, la presentación visual ni la paridad de toda la tarea `0x32`. La investigación de crecimiento y memoria liberada del ciclo `0x03` permanece en `C10_TEN_DAY_GROWTH.md`; no se elimina la guardia ante el crecimiento no determinista.

## Auditoría de ramas todavía pendientes

- La fundación asignada y aún sin iniciar está conectada al despacho; contrastes y límites de crecimiento, reloj y S05 en `C10_FOUNDING_SCHEDULER.md`.
- La aproximación inicial de la misión pirata ya ejecuta la reposición de bandas; véase `C10_PIRATE_MISSION_LIVE.md`. El encuentro posterior de fase 1, que debe preparar la guarida y el asalto, sigue pendiente.
- En los cinco escenarios de pasajero con barco y capitán, esas imágenes sintéticas se aportan tanto al x86 como a Rust para el despacho y se retiran antes de guardar el S05 base. Por ello se acreditan la tarea y sus efectos en S05, no la persistencia de ese barco sintético. La flota sigue siendo necesaria para la consulta; no se exige la tabla de destinos piratas.

La identificación anterior procede de la lectura de los despachadores y modelos existentes, no de una nueva validación integral en partida.
