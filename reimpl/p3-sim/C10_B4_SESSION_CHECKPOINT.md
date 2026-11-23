# Checkpoint B4: sesión, interrumpido por orden del usuario

Estado: trabajo incompleto en rama propia, no integrado ni aceptado. Se detuvo
la compilación/prueba `cargo test -p p3-sim --lib
pirate_b4_session::tests::live_b4_publishes_and_saves_atomically` antes de que
finalizase; no hay resultado de compilación ni de esa prueba.

El commit anterior `2b8143f0` prepara transaccionalmente B4 con las vistas
vivas y corrige la ciudad del Ältermann. Tiene 200 casos x86 aprobados, un
ciclo de alta→sustitución→S02/S05, comprobación de conflicto sin cambios y
crecimiento forzado de ambos pools. El x86 adapta el heap y banco de nombres;
no acredita la cadena completa desde 514920 ni una partida real.

Trabajo guardado en este checkpoint:

- `pirate_b4_session.rs` es un primer borrador de `Sim::dispatch_live_pirate_b4`.
  Reúne vistas y recursos, prepara B4, valida proyecciones y publica la copia,
  con un test que debe comprobar barrera de operaciones y persistencia S05.
  No está compilado ni contrastado. Revisar en particular coherencia de
  proyecciones de carga y convoy, alta/baja de barcos y estado de retratos,
  así como la publicación cuando crecen los pools.
- Se modificó `pirate_b4_live.rs` para conciliar solo los campos serializados
  de ranuras libres antes de activarlas; su regresión se adaptó, pero ambos
  cambios siguen sin probar.
- `lib.rs` registra los tres módulos solo en esta rama para permitir compilar.
  Revisar e integrar ese registro explícitamente; no copiar `lib.rs` encima
  de los cambios del coordinador.

Siguiente paso tras reanudar: revisar el diff y compilar el test interrumpido;
corregir sus errores, repetir la regresión `c10_pirate_b4_x86` y contrastar la
sesión con S05 y x86. Integrar el método con el bombeo sin extraer B4 de la
cola antes del commit atómico y conservar la barrera S05. No hacer cherry-pick
de este checkpoint como si estuviera terminado. Trabajo en el PC de Sergio.

## Reanudación y cola (PC de Sergio)

El borrador se compiló y el test de sesión con AUTO pasa. B4 se despacha desde
la cola de `Sim` con publicación transaccional; si faltan recursos se conserva
B4 y las operaciones posteriores. El recorrido nativo de asalto vacío guarda
y recarga después de B4. Los 200 contrastes x86 de B4 siguen siendo aislados: no
son una comparación x86 de toda la cadena ni una prueba en partida.
