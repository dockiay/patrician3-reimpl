# C10: carga de batallas navales y reloj

Al cargar S05, `Sim::from_save_bytes` analiza el bloque de batallas y conserva la imagen de origen si hay al menos un objeto naval. Una batalla cargada queda pendiente: las entradas del reloj con comerciantes y del bombeo rechazan el avance antes de ejecutar operaciones. La marca permanece mientras `Session::open_with_paused_battle` reconstruye y mantiene pausada la batalla.

Al resolverse y destruirse la batalla, la sesión comprueba antes de publicar que el gestor resultante no tenga ranuras activas, proceda del bloque cargado y deba escribirse; solo tras esa comprobación instala el gestor y libera el reloj. Una validación fallida conserva la marca. El guardado de una batalla pausada sigue permitido y su recarga vuelve a marcarla como pendiente.

Los errores del analizador del gestor se comunican al cargar; el ataque urbano no reconstruido conserva su rechazo específico. Un bloque con listas auxiliares y sin batallas activas mantiene el tratamiento anterior.

## Pruebas y límites

`c10_saved_battle_guard` utiliza un S05 local y una batalla de un fixture existente para comprobar bloqueo de los dos caminos del reloj, preservación de operaciones, rechazo de gestores ajenos o activos, recarga y errores `Trailing`, `Truncated` y variante de ataque. `initial_battle_can_be_opened_paused_and_roundtrip_without_advancing` comprueba la marca al abrir y recargar una sesión; `session_removes_ai_sunk_ship_and_reloads` comprueba su liberación tras la destrucción real ya cubierta por el paquete naval.

Estas pruebas comprueban protección del estado Rust y estructura S05; no acreditan por sí mismas la paridad completa del combate con el ejecutable ni validación en partida. Requieren `P3_SAVES` y, las pruebas de sesión, `P3_CPR`.

`idle` y sus campos permanecen públicos para pruebas de bajo nivel: manipularlos directamente o invocar `p3_core::tick` sobre `IdleSystems` evita este control y no constituye un avance completo de `Sim`. Los persistidores parciales no certifican un S05 completo.
