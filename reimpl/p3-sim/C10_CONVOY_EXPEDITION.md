# C10: planificación de expediciones al salir del puerto

PC de Sergio, 4/11/2026. El día2 de AUTO conservaba convoy2+0x14=2 frente a0 en
el original. El bucle naval ejecutaba el estado2 y la transición al15, pero su
anfitrión ignoraba `cv_503b50`.

`ships::convoy_expedition` porta `0x503B50` y `0x534F10`. Para dueño IA borra el
bit2 antes de las guardas. Requiere ambos bytes de mundo+0x8A/+0x8B y cliente de
red inactivos, líder válido y cada miembro con casco >= máximo-(máximo>>4) y
carga <=5000. Ambas comparaciones son con signo; 5000 se admite. Devuelve la
operación1E y cuatro1F, conservando cinco consumos de la CRT original. Usa el
índice de convoy guardado en el líder, no el índice recibido por la función.

El selector usa los primeros15 centros, ubicación+4 y tiempo+8. En la búsqueda
inicial acepta sólo tiempo estrictamente anterior al reloj. Al agotar esa
búsqueda BL queda1: el fallback para centros aún no descubiertos recorre sólo
semilla%15..15, sin envolver. El selector de puntos recorre desde
100+semilla%(número-100), y luego desde100. Ninguno libre devuelve FFFF/FFFF.

Recursos exactos: pares u16 de la tabla0x673858 para ubicación<30, y pares u16
`Scripts/Expeditionen.dat`, cargados por0x534750 a0x6DF2E0, número en0x6DF2E4.
Los bits conocidos son los del comerciante+0x4D8, no la unión global de mapas.

Prueba `x86_c10_convoy_expedition.py` y Rust del mismo nombre: 640 casos,351
planes, comparando flags, cinco operaciones completas y estado CRT. Se ejecutan
las dos funciones, constructor0x535740 y rand0x639C03 originales. Sólo TLS,
consulta de comerciante y publicación de operaciones se interceptan. Hay
comprobación de rechazo atómico ante contexto ausente. No hay prueba en juego.
El puerto devuelve órdenes; su despacho1E/1F y conexión a navegación se revisan
por separado. El callback `ConvoyHost::plan_expedition_with_world` permite esa
conexión sin cambiar el contrato de los oráculos antiguos del tick.
