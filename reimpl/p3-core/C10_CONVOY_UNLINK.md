# C10: sacar un convoy atacante de la lista activa

`ships::convoy_unlink::unlink` prepara `0x0050A4E0` sobre copias: encuentra el convoy en la cadena activa, lo desenlaza, elimina el líder válido del orden por x con `0x0050D040` y devuelve la solicitud de refresco de interfaz. La entrada y salida conservan las imágenes originales. Rechaza sin cambios una cadena cíclica o un índice que coincide con la cabecera pero queda fuera del pool: el original intentaría leer memoria ajena en ese caso.

`cargo test -p p3-core --test c10_convoy_unlink_x86` compara 120 escenarios frente al ejecutable inglés: cabeza, interior, ausencia, convoyes inválidos, líderes válidos o ausentes y enlaces espaciales consistentes o parciales. Se comparan gestor, barcos, convoyes, retorno bajo y orden de refresco; la interfaz se intercepta. Requiere `P3_EXE` y `P3_ORACLE_PY` y se omite si faltan.

El ayudante no conecta todavía el ataque de misión, la escena o S05; no es una prueba en partida.
