# C10: guarida pirata con plano y convoy existentes

`pirate_group_runtime::prepare_hideout_existing` prepara la parte de `0x00514530` en la que la banda ya tiene un plano y un convoy válido. Sobre copias, traslada sus barcos de reserva al convoy en el orden original, actualiza los enlaces de banda, convoy y barcos, y equipa cada barco según el nivel de la banda y el astillero de la ciudad del Ältermann. Devuelve el formato `Prepared<H>`; el anfitrión no recibe callbacks simulados y las entradas se conservan si falla la preparación.

Si faltan el plano o el convoy se devuelve un error explícito: la construcción del plano, la posible creación de barco y la creación de convoy no forman parte de esta ruta. También se rechazan reservas cíclicas y flota o ciudad inconsistentes. El llamador debe publicar simultáneamente las imágenes y el estado de banda, y después iniciar o reforzar la escena de ataque correspondiente.

## Contrastes

- `cargo test -p p3-sim --test c10_pirate_hideout_existing_x86 -- --nocapture`, con `P3_EXE` y `P3_ORACLE_PY`: 64 escenarios ejecutan `0x00514530` y sus llamadas reales `0x0052BB00` y `0x0051A4E0` en x86 aislado. Se comparan bandas, barcos, convoyes y ciudades completos; varían reservas de cero a tres barcos, cuatro niveles de astillero y cuatro niveles de banda. Otras pruebas comprueban rollback ante plano ausente, convoy ausente y reserva cíclica.
- `cargo test -p p3-sim --test c10_pirate_replenish_s02 -- --nocapture`, con `P3_SAVES`: sobre una copia de AUTO se crean dos barcos de reserva, se guarda y recarga S02, se aporta un plano y un convoy existente y se realiza el traslado; banda, cadena y convoy sobreviven a una segunda recarga. El convoy de la prueba se toma del guardado como estado sintético de entrada.

El oráculo comprueba el hash del ejecutable inglés, configura `FPCW=0x027F` y ejecuta sin interceptar callbacks en esta ruta. Esto no acredita el despacho completo de `0x32`, la apertura o refuerzo del combate ni una partida.
