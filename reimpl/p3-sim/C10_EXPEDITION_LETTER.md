# Carta de regreso de expedición

`expedition_return_letter` prepara los bytes de la carta 0x63 sin modificar
el pool. Lee los fragmentos y nombres del ejecutable inglés identificado por
SHA-256; recibe por separado barco, ciudad, hallazgos, importes y resultados
comerciales que deberán proceder de la transacción de la expedición.

El tramo x86 `4E7099..4E79A3` se ejecuta en 32 casos: cero a cuatro centros,
aldeas mezcladas, punto sin centro, tormenta, ataque, dinero y capitán. El
oráculo intercepta el retorno físico, las asignaciones, las operaciones de
`CString`, el formato de números y la entrega; no pretende contrastar aquí
esas dependencias. Compara exactamente los bytes y la longitud declarada de
la carta. Otros 12 casos ejecutan `4D6530` y el serializador x86 contra la
entrega nativa, con destinatarios humanos/IA, preferencias, pool ampliado y
payload completo o truncado. La cabeza del destinatario y el pool se confirman
en una sola operación; las direcciones de heap se normalizan conservando su
presencia. En las enumeraciones de tres o cuatro centros el original escribe
comas sin sumarlas a la longitud, por lo que el payload guardado puede quedar
sin terminador o truncado; el porte conserva esa diferencia.

El sufijo de aldeas al que apunta `0x6B9634` está en memoria virtual, no en
los bytes del PE. Se suministra explícitamente; si se necesita y falta, se
rechaza el resultado. El fixture x86 cubre tanto cero como un byte no nulo. La entrega
S05 está preparada, pero aún debe conectarse a la tarea de retorno completa.

Prueba con `P3_EXE` y `P3_ORACLE_PY` reales:

```
cargo test -p p3-sim --test c10_expedition_letter_x86
```
