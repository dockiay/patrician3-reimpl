# C10: puente de misión pirata desde la sesión

`Sim::prepare_live_pirate_attack` comprueba la tarea en cabeza, recursos de ciudad, bandas y sus vistas, contadores y cadenas de barcos, y prepara el ataque sobre copias de la navegación. Cuando cambia un barco o convoy, exige que las vistas de expedición y la fábrica afectadas coincidan con la navegación. Una discrepancia se rechaza antes de publicar. El llamador debe averiguar si hay escena, completar sus efectos y publicar la transacción; este puente todavía no está conectado al reloj ni a S05.

La prueba `cargo test -p p3-sim --lib live_wait_uses_save_views`, con `P3_SAVES`, parte de AUTO y comprueba una espera y un convoy atacante sintético, así como el rechazo de rumbo ausente y vistas desincronizadas sin mutar la navegación. No abre el juego ni compara esta unión con el original: los contrastes x86 de los dos tramos están documentados en `C10_PIRATE_ATTACK_STAGE.md`.

El puente `Sim::prepare_live_pirate_scene` verifica además que tick, jugador, ciudad del concejo y posición coinciden con la sesión antes de reservar. Preserva el jugador local como u32, incluido el centinela sin jugador para la vía IA. Sigue sin publicar ni ejecutar el dispatcher.
