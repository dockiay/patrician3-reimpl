# C10: contratos existentes de centros

El cuerpo completo de `0x52C6C0` tiene además un porte SEH independiente en
`economy::seh_pending`, registrado como `probada_emulador`: 240 casos base y
240 nuevos repetidos el 11/11. Este adaptador de simulación tiene el alcance
descrito abajo. [Aclaración y límites del cuerpo SEH](../../research/systems/revision-reimpl/CONTRATO_CONVOY_52C6C0_2026-11-11.md).

`trade_center_contract::prepare` prepara la entrega o compra contra un
contrato existente de 0x52C6C0 sobre copias. Reproduce selección por comerciante,
ambas fases, lotes, presupuesto, reparto por barcos, reducción y baja del
contrato, sus contadores y el registro económico. El anfitrión aporta carga,
capacidad y texto y debe publicar todas las vistas juntas.

El oráculo ejecuta 96 casos x86 de contratos sintéticos, con dos direcciones,
dos fases, dos mercancías, dos tamaños de lote, tres cantidades y dos presupuestos,
incluidas listas con otro contrato anterior. La carga, el
reloj de fechas, la liberación de memoria y el texto están interceptados.
Coinciden centro, convoy, comerciante, barcos y contrato; se prueba también el
rollback cuando falla el informe. `create` prepara por separado la creación
aleatoria con LCG y población: 40 casos x86 comparan registro y contadores.
Su texto debe prepararlo el anfitrión. La variante `prepare` mantiene el rechazo
explícito de creación para que el llamante no se salte ese efecto.
