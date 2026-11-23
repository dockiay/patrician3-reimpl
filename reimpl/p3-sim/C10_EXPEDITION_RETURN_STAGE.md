# C10: transacción de regreso de expedición

`expedition_return_stage::prepare` prepara en copias el recorrido de destinos, el desgaste, el cierre de convoy y, cuando hay supervivientes y dueño humano, la carta 0x63. El importe inicial se captura antes del comercio y el devuelto después del desgaste. El anfitrión explícito debe proporcionar la flota tras el comercio y sus textos; cualquier error descarta el resultado.

`c10_expedition_return_stage_x86` ejecuta cuatro escenarios sin destinos válidos: daño original seguido de 502B20 y serializadores x86 de barcos, capitanes y cartas; compara imágenes, avisos, resultado del azar y serialización. Incluye daños, tormenta/ataque y pérdida total. Comprueba además rollback si falta el texto de carta humana. No ejecuta el dispatcher ni la carta 0x63 como parte de esa misma ejecución; estos componentes tienen contrastes separados. El host de comercio real y el cableado de la transacción a la sesión y al planificador aún están pendientes. Sin validación en partida.

La selección del capitán se hace tras 502B20, usando el líder vigente y un banco vivo explícito. Cinco escenarios x86 incluyen ahora líder hundido y sustitución por otro barco con distinto capitán; se comprueban las imágenes y el nombre seleccionado. La carta humana completa aún no forma parte de esa ejecución x86 encadenada.

La carta vacía de contratos usa dos cadenas leídas del ejecutable, sin omitirlas cuando no hubo visitas. Nueve escenarios sin waypoints pasan contra `4E6A54` completo, incluido cierre, carta y serializadores x86; la ruta y el banco de nombres se adaptan de forma explícita. Cinco escenarios pasan además por el planificador y sesión Rust, contrastados con el resultado del x86. El comercio de centros aún se rechaza expresamente.

El oráculo ampliado también ejecuta una marca sin centro coincidente, con primera recompensa 0x56, ventana, desgaste, cierre y 0x63 en la misma memoria x86. La capa de sesión usa ahora el anfitrión nativo transaccional de centros, con precios y reputación leídos del EXE. Se rechazan las ramas de comercio y contrato cuyo texto todavía no está conectado; no se omiten sus efectos.

El recorrido integrado incluye compra, venta y cumplimiento de un contrato existente, con descubrimiento, ambas cartas 0x1C, sustitución de líder, carta 0x63 y serializadores x86. Las pruebas fijan FPCW=0x027F. Dos pruebas adicionales usan una partida real: hundimiento total o retorno, despacho y reconciliación del reloj, guardado S05, recarga de barco/convoy y repetición de guardado idéntica. Las pruebas no son validación en partida.

También se contrastan el alta de un contrato nuevo y una aldea de identificador 30 con 31 posiciones. En el caso de aldea se suministra explícitamente el sufijo dinámico vacío observado en ese perfil del oráculo; no se afirma que lo sea en todas las sesiones.

La prueba de reloj avanza un tick con `advance_with_merchant_daily`, que ejecuta la reconciliación ordinaria. Una variante completa un contrato de venta en un centro real: compara tras recarga el centro y su lista de contratos, comerciante, mensajes, barco, convoy y ruta, y verifica que un segundo guardado sea idéntico. El escenario usa AUTO con los datos de expedición editados solo en memoria.

Si dos regresos modifican el mismo centro y se guarda entre ambos, el scheduler conserva la versión anterior como propia. Una regresión de dos ticks y dos expediciones verifica la recarga del segundo S05; sin registrar la versión, reproduce el conflicto de centro modificado.
