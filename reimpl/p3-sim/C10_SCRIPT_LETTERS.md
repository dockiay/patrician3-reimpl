# F7: clases de carta, preferencias y avisos

PC de Sergio, 4/11/2026. `WareVerstecken.p2m` alcanzaba una carta F7 de clase 2
(`0x3E`) después de las lecturas `0x44` y `0x11`. Su byte de notificación es cero;
el mensaje de error antiguo agrupaba una clase no admitida y una notificación.

La preparación admite ahora clases 0..5 (`0x3C..0x40`, `0x71`). La entrega utiliza
las preferencias y la retención del publicador original. No se fuerza la entrega
de una carta desactivada por el comerciante. Los descriptores, respuestas y textos
se conservan en S05, y la preparación sigue siendo transaccional.

El formato `%7` que usa ese guion pertenece a los marcadores `%0..%7`: seleccionan
una de dos cadenas por `merchant+0x1A & 1`, con índice de comerciante reducido a
u16. Los 16 textos se leen de la tabla `0x006C3040` del ejecutable inglés local
con hash comprobado. No se versionan textos comerciales. Un comerciante fuera
del rango aporta cadena vacía como el original. Los marcadores 8 y 9 no tienen
entradas válidas en esa tabla, aunque el salto del intérprete llega al mismo
código; permanecen rechazados.

Con el byte de notificación activo se conserva el aviso `0x12` de `0x548CA0` en
`host.mission_script_notifications`, junto con la petición de refrescar el correo
para el destinatario local. Este efecto permanece pendiente de presentación y
sobrevive a la reconstrucción interna de la sesión. No se abre una ventana ni se
simula que ya fue mostrada. El refresco se contrasta con la consulta visual
habilitada en el oráculo; las funciones finales de interfaz son fronteras
interceptadas. La carta y las variables sí se persisten; las peticiones visuales
pertenecen al anfitrión, no al formato PAT.

Evidencia:

- 240 construcciones del intérprete original con clases 0..5, respuestas,
  destinatarios y byte de notificación; variables, descriptor, texto y argumentos
  del aviso coinciden.
- 360 entregas por `0x4D6530` y serializaciones `0x4D62B0`, con destinatarios
  directos/globales/todos los humanos y preferencias 0..5.
- 24 cadenas de planificador, F7, entrega y serializador originales con imágenes
  de comerciantes de un guardado real, las 16 variantes gramaticales, clases,
  preferencias y avisos. Coinciden pool, variables, tareas y peticiones de correo;
  se comprueban doble guardado y recarga. Un comando posterior rechazado no
  publica la carta ni su notificación.

```powershell
cargo test -p p3-core --test c10_script_letter_x86
cargo test -p p3-state --test c10_script_letter_delivery_x86
cargo test -p p3-sim --test c10_script_letter_classes_live_x86 --test c10_mission_live_x86
```

Requieren `P3_EXE`, `P3_ORACLE_PY`, `P3_SAVES` y, para las regresiones con recursos,
`P3_CPR`. `Session::configure_battle_texts` prepara también esta tabla del mismo
ejecutable y la transfiere al activar la flota.

## Difusión con respuestas y destinatario urbano

En `0x004EDCFC..0x004EDE84`, F7 con respuestas, clase distinta de 5 y
destinatario negativo inicializa ocho variables de respuesta y luego ocho
variables de destinatarios a `-1`, antes de leer el primer destinatario. Por
ello los valores iniciales `-1`, `-2` y `-3` siguen los tres la difusión a
comerciantes humanos: el supuesto camino urbano queda inalcanzable en esa
rama. Se escribe un comerciante por variable y se incrementa el índice de
respuesta en cada descriptor, incluso si sus preferencias filtran la carta.
El puerto conserva el orden de escrituras, incluidos solapamientos, y rechaza
transaccionalmente una tabla de variables insuficiente. Una notificación se
conserva incluso sin destinatarios humanos.

Para F7 sin respuestas o de clase 5, `0x004D6530` sí recibe `-3`: recorre las
oficinas de la ciudad por `Town+0x784` y `Office+0x2CA`, y entrega por dueño
`Office+0x2C4`. La implementación de cartas de guion conserva el orden y los
posibles duplicados, con el filtrado habitual de humanos y preferencias; el
refresco de correo consulta la ciudad de la notificación. Se rechazan cadenas
cíclicas e índices inválidos. El `post_message` genérico sigue sin portar `-3`.

La rama con respuestas aparece en los guiones reales ingleses y españoles de
`Warentausch`, `WettbewerbKarte` y `WettbewerbRennen`: sus segmentos crean
`-1` mediante una resta antes de F7. También se crean valores `-3` para F7 en
`PluenderndeHorden` y `WarenAufkaeufer`. La lectura estática de esos segmentos
no demuestra cuándo se alcanzan desde el inicio del guion ni sustituye una
prueba en partida.

- `c10_f7_broadcast_x86`: 36 escenarios, 180 cartas capturadas, incluidas
  variantes negativas, ningún humano, más de ocho destinatarios y variables
  solapadas. Compara las imágenes de variables, textos, descriptores y avisos.
- `c10_f7_delivery_live_x86`: ocho escenarios con el planificador, intérprete,
  listas de oficinas, entrega y serializadores x86; compara tareas y mensajes
  y verifica doble guardado y recarga S05. Incluye una ciudad con varias
  oficinas, control humano/IA y preferencias.
- Las regresiones de clases, entrega y constructor F7 previos también pasan.

Quedan formatos y comandos F7 no portados y el servicio genérico de cartas
urbanas fuera de esta ruta. Los contrastes son x86 aislados, no acreditan una
partida visual ni el recorrido completo de 30 días.
