# C10: carga y guardado x86 de ciudades antes de B5

En el PC de Sergio se comparó la captura inmediatamente anterior a la B5
de `fps-cache/save/Ein/AUTO.pat` con su carga y guardado en Rust. El oráculo
ejecuta aisladas `town_deserialize` (0x0051B520) y `town_serialize`
(0x0051B330) para cada ciudad, incluidos sus edificios. El emulador sustituye
las dependencias genéricas de reserva de memoria y copia; no ejecuta el juego
ni el cargador completo de la partida.

Los flujos serializados de las **24 ciudades** coinciden byte a byte entre
las dos rutas. Tanto el x86 como Rust recalculan los umbrales Town+0x628 y
Town+0x62C en las mismas 12 ciudades: 1, 3, 4, 6, 7, 9, 10, 12, 13, 14,
16 y 23. El x86 también reordena los dos nodos de la cola de la ciudad 9,
igual que Rust. Las otras ciudades conservan íntegro su flujo original.
Estas diferencias no proceden de B5 en esta captura.

```text
python reimpl/oracle/x86_c10_b5_town_reload.py --binary EXE_INGLES \
  --payload CAPTURA_ANTES_B5 --native-payload CAPTURA_RUST_GUARDADA \
  --require-match
```

Ambas capturas son contenidos S05 descomprimidos locales, no versionados.
El oráculo emite únicamente digests y posiciones de diferencias. SHA-256:

- captura original: `2028fe334cd0a39cf12d37352d33f1251f5b9fefcb53456a0294412d50b1f14b`;
- captura Rust: `0bd35debfe4b7c990c0bbf41a39169b245e6611ffc805201d120fd175250aeae`;
- EXE inglés: `fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece`.

El informe local está en `coordination-runtime/c10-b5-town-reload.json`.
Este resultado cubre esta captura y estas rutinas, no demuestra paridad de la
sesión completa ni validación en partida.
