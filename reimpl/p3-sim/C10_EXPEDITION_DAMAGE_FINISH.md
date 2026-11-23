# C10: desgaste, cierre y serializadores de expedición encadenados

`x86_c10_expedition_damage_finish.py` ejecuta en una sola memoria el tramo
`0x4E6C78..0x4E7099`, la función original `0x502B20` y sus dependencias,
y los serializadores originales de barcos `0x5177D0`, capitanes `0x4FDE30`,
comerciante `0x4F4010` y cartas `0x4D62B0`. La prueba
`c10_expedition_damage_finish_x86` compara los resultados con
`expedition_damage::prepare` y `expedition_finish_stage::prepare`, incluidos
bytes serializados de barcos, capitanes, comerciante y cartas, imágenes de
convoy y sucesos de desgaste. Nueve escenarios cubren IA y humano, desgaste,
tormenta, ataque, supervivientes, cambio de líder con capitán propio y pérdida de toda la flota.

El buscador x86 `0x445010` se intercepta con la ruta del planificador Rust;
se serializa primero con el puntero x86 y se normaliza después su dirección
al token nativo. Los servicios de montón/memoria y las notificaciones de la
interfaz están adaptados en el emulador. El serializador de comerciante se compara sobre el registro fijo sin grupos.
La pila del tramo inicial está preparada por el fixture: aún no ejecuta el
bucle de visitas, el texto de la carta 0x63 ni el dispatcher completo de la
tarea `0x12`. No acredita una transacción integrada de la sesión ni juego.

Depende del módulo `expedition_damage` de `a25bc00f`, del núcleo 502B20 de
`c19914b6` y del host/puente `6e2f3aad` y `a546e339`. Con las variables
`P3_EXE` y `P3_ORACLE_PY` configuradas:

```
cargo test -p p3-sim --test c10_expedition_damage_finish_x86 -- --nocapture
```

Siguiente: ejecutar visitas, cierre, carta 0x63 y persistencia S05 en la misma
transacción del scheduler con grupos de barcos y recarga.

Las tres sondas aisladas de finish y damage_finish fijan FPCW=0x027F.
Con este perfil pasan los 96 escenarios del núcleo, los 14 del host y los
nueve encadenados de desgaste, cierre y serializadores.
