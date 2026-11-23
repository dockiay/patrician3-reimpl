# Sufijo de aldeas de la carta de expedición

`Session::set_expedition_indigenous_suffix(&[u8])` suministra los bytes del
sufijo dinámico de la carta 0x63 después de configurar el EXE inglés mediante
`configure_battle_texts`. Sin EXE configurado devuelve error y no modifica la
sesión. No supone una cadena predeterminada para el perfil general.

El sufijo es contexto del proceso, no un campo S05. Se conserva cuando la
misma sesión reconstruye la simulación mediante `enable_full_fleet` o al
reanudar navegación. Guardar S05 no lo altera ni lo introduce en el archivo;
una sesión abierta desde cero debe recibir otra vez los recursos y el sufijo.
El perfil headless concreto observado puede pasar `&[]` si declara esta
hipótesis; los detalles y límites de la evidencia están en
`p3-sim/C10_LETTER_63_OBSERVATION.md`.

Con `P3_SAVES`, `P3_CPR` y `P3_EXE` configurados, la prueba
`cargo test -p p3-mvp --test expedition_suffix_session` verifica error sin
recursos, uso de bytes no vacíos, S05 inalterado, reconstrucción de la flota,
y recarga nueva con configuración explícita. El formato de carta con sufijo
variable tiene contraste x86 en `c10_expedition_letter_x86`.
