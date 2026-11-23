# Conectar la financiación de la IA (tarea 0x04, `raise_cash`) a p3-sim

Estado (9/11, nube): el puerto `p3_core::ai::cash::raise_cash_or_go_bankrupt` (0x004F4E50) ya tiene paridad x86 (`tests/x86_ai_cash.rs`, 4.000 casos), igual que `ai::work::queue_building_work` (0x004F5A20). Lo que falta es la conexión: `ai/scheduled_turn.rs` (`Probe::raise_cash`) devuelve «financiación de IA no conectada» cuando no es modo especial ni red, y 7 de los 9 guardados de prueba se detienen ahí a los ~30 días.

## Plan (mismo patrón que `p3-sim/src/merchant_ai_decisions.rs`, que ya hace vivienda y gremio)
1. `Probe::raise_cash` pasa a marcar `cash: bool` en `TurnPreparation` (como `housing`/`guild`) en lugar de `pending(...)`.
2. En `merchant_ai_decisions::plan` añadir la rama `cash`, con un puente `CashHost`:
   - `enqueue_operation` → `Operation(*r)`.
   - `rand` → copia de `self.investigation_rng` (estado CRT); `merchant_ai_preview` devuelve el estado nuevo y el aplicador (lib.rs ~1862) lo guarda.
   - `queue_building_work` → `ai::work::queue_building_work` con `WorkEnv` construido de los datos vivos: `WorkTown{records,stride=+0x7AC}` desde `sites[i].records`, **celdas por ciudad** (`site.cells`; hoy `WorkEnv.cells` es una sola tabla compartida: hay que pasarla a por ciudad), `tasks` desde `employment_businesses` (+4 empleados, +6 tipo, +8 siguiente, +0xC), `offices` desde `scheduled_office_images` (+0x2C6, +0x2C8, +0x2CC, +0x2CE, 86 palabras desde +0x2D0), `office_head=Merchant+0xC`, `guild_mask=Merchant+0x468`. `is_unclaimed` = 0x004DAFE0 (planificador, tareas 0x36 de la ciudad) y `building_cost` = `site.building_base_value` + `auction::building_sale_price` (como `auction_runtime.rs` ~l.185).
   - `has_building_in_auction` = 0x004DB220: recorre la lista del planificador (cabeza `+8`, siguiente `+4`), tareas 0x36 con ciudad `+0x15` y tres palabras `+8..+0xD` (`& 0x7FFF`) cuyo registro de edificio (`0x51F1F0`) tiene propietario `+2` = tarea. Falta portarla con oráculo (no hay fila en functions.tsv).
3. El despachador de operaciones no maneja 0x28 (obra de venta del edificio de la IA) ni 0x65 (quiebra, `ai::bankrupt` / 0x004F5D80): hoy caerían en `dispatch_offer`. Hay que añadir esos manejadores en `c08_operations.rs` contrastados con el despachador 0x535760 (`oracle/x86_c08_ops_live.py`). 0x29 ya está.
4. Pruebas: oráculo del turno completo para la rama `cash`, guardar/recargar y avance de 120 días sobre los 9 guardados.

## Límites conocidos
- `queue_building_work` lee una ranura de pila sin inicializar en su primera comparación (`WorkEnv.residue`). Con 0 el primer candidato nunca se elige; hay que decidir el valor (cualquier distinto de cero da el mismo resultado salvo desbordes) o emular el residuo real desde 0x004F4E50.
- Los guardados de prueba y el despachador 0x535760 en emulador están en el PC de Sergio; desde la nube no se puede ejecutar el avance de 120 días.

## Seguimiento 9/11 noche (nube): ya no para en 0x04
El turno 0x04 se detenía antes de llegar a la financiación: `merchant_ai_preview` necesita el jugador local para el
factor de reputación (`0x004F42B0` con modo `& 0xE == 0`) y el cargador lo dejaba sin definir. Ahora
`Sim::from_save_bytes` lo fija como el opcode 0xC1 del original (`p3-sim/src/load_start.rs`, paridad con
`oracle/x86_load_start.py`). Con los 9 guardados y los recursos de `real_saves` (oficina humana, CPR, CRT 123), ninguno
para ya en 0x04; donde se detienen ahora:

| guardado | días | primer error |
|---|---|---|
| fps-cache/AUTO | 25,8 | encuentro pirata pendiente de conectar |
| visibility/AUTO | 6,8 | encuentro pirata pendiente de conectar |
| ConvoyAvista | 7,6 | encuentro pirata pendiente de conectar |
| ConvoyHecho | 6,8 | encuentro pirata pendiente de conectar |
| ConvoyMar y TEST25_ConvoyMar | 2,8 | encuentro pirata pendiente de conectar |
| ConvoyPrueba | 3,4 | encuentro pirata pendiente de conectar |
| TestAstiller (×2) | 1,74 | encuentro pirata pendiente de conectar (10/11: antes 0,06 por la tabla de centros comerciales 0x673858 sin cargar) |

El encuentro pirata lo lleva el hilo de la batalla pirata (C07). Las pruebas que no reconstruyen la flota se detienen
en 0x03 («flota viva no reconstruida») o 0x1B (recursos de guion), que dependen de su configuración, no de 0x04.
