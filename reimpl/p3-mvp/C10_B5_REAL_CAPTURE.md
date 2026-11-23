# C10: contraste de B5 observada en AUTO

PC de Sergio. Se compara la operación observada `[181,0,320,3,0]`, despachada
al tick `125734773` por el original, con el despacho nativo tras cargar la
captura inmediatamente anterior. Los payloads comerciales locales se
verifican por SHA-256; también se verifican el ejecutable inglés y el CPR.
La cola no se serializa: el test inyecta exactamente la operación observada.

La regresión `c10_b5_real_capture` compara en los siete bloques cada cambio
como `(desplazamiento, byte previo, byte posterior)`. Coinciden los 77 cambios
del bloque de barcos; los otros seis bloques no cambian. El guardado nativo se
repite y produce el mismo fichero. La referencia x86 y sus observaciones se
registran en `reimpl/oracle/evidence/c10_b5_fps_original.json`.

Límite: recargar la captura y guardar sin ejecutar B5 ya produce diferencias
en `game_world` y `ships` respecto a la captura de la ejecución original. La
prueba acredita igualdad exacta de las deltas de esta operación, no igualdad de
los payloads completos ni de todos los posibles estados de B5, y no es QA en
partida. Los tres payloads originales comparados y su verificación de
procedencia permanecen en `coordination-runtime`.

Para ejecutar el contraste, definir `P3_C10_B5_DIR`, `P3_CPR` y `P3_EXE` y
correr `cargo test -p p3-mvp --test c10_b5_real_capture -- --nocapture`. Sin
esas variables, la prueba informa que se omite.

Diagnóstico de esa base: en `ships`, las diferencias se limitan a los bytes
del puntero de ruta de 57 registros de barco; no cambian cabecera, capitanes,
convoyes ni cola. En `game_world`, 12 ciudades difieren en los campos
`Town+0x628/+0x62C` de umbrales de precio; el constructor nativo recalcula la
tabla `Town+0x4F0..+0x66F` al cargar. Además se observan dos entradas
reordenadas en la cola de la ciudad 9. Falta ejecutar una carga y un guardado
x86 de la misma captura para decidir si esas diferencias urbanas concuerdan
con el lector original. La igualdad de la delta de B5 no resuelve esa cuestión.

Contraste posterior: `reimpl/C10_B5_TOWN_RELOAD.md` y el oráculo del commit
`23ccd5cc` comparan la carga y el guardado x86 de la misma captura con el
baseline nativo: 24 de 24 flujos urbanos completos coinciden byte a byte,
incluidos umbrales y la cola de ciudad 9. Se ejecutan los serializadores
urbanos y de edificios originales, con heap y memcpy emulados; no es una carga
completa del juego. La reasignación de punteros de rutas navales permanece
fuera de esa comparación urbana.
