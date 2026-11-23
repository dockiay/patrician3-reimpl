# C10: reposición de barcos de banda al acercarse a la guarida

`pirate_group_runtime::prepare_replenish` prepara una pasada de `0x005144A0` sobre las cinco bandas presentes y devuelve copias de banda, flota, capitanes, gestor y anfitrión. No modifica las entradas. Reutiliza el alta nativa `0x00516170`; acepta la dificultad de la partida, la tabla de límites del EXE y el calendario como datos explícitos. Rechaza reservas cíclicas y descarta la transacción si faltan los recursos de alta. Los pools pueden crecer en bloques de 64; las rutas S02 de las ranuras activadas se deben publicar con las imágenes de flota.

El original conserva bits altos de un puntero al calcular el tipo del siguiente barco. `0x00516170` utiliza su byte bajo; el puente entrega ese byte al anfitrión. El contraste incluye varias altas consecutivas.

## Pruebas

- `cargo test -p p3-sim --test c10_pirate_replenish_x86 -- --nocapture`, con `P3_EXE` y `P3_ORACLE_PY`: ocho escenarios frente a `0x005144A0 → 0x00516170` x86. Incluyen bandas presentes, destino diferente, plano ya cargado, reserva parcial, varias altas, dos bandas y crecimiento 8→72 de barcos y capitanes; también se verifica rollback al faltar recursos. Se comparan cabeceras de bandas, gestor, flota, capitanes y ciudades.
- `cargo test -p p3-sim --test c10_pirate_replenish_s02 -- --nocapture`, con `P3_SAVES`: una copia de AUTO conserva dos barcos, sus capitanes y la banda al escribir y recargar S02. Las rutas se concilian a partir del `NavigationSnapshot` con `ShipsModel::apply_hanse_factory_navigation`.

El oráculo ejecuta el código original en x86 aislado y adapta el heap y el lookup del banco de nombres; `FPCW=0x027F`. Los límites y bancos son entradas de la prueba. La conexión posterior a la fase 0 de `0x32` se documenta en `C10_PIRATE_MISSION_LIVE.md`. El encuentro de fase 1, los avisos y el combate siguen pendientes; esta entrega no es una prueba en partida.
