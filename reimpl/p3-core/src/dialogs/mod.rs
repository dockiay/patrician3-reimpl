//! Diálogos de la interfaz con paridad x86 (hilo «Portar diálogos de la interfaz», 11/11/2026): solo la parte que decide algo (qué operación
//! se envía, qué estado del diálogo se fija, qué ventana se abre); el dibujo y las plantillas de controles quedan en la capa de presentación
//! del cliente. Forma común: imagen de memoria + [`OpsHost`] para las llamadas del original (widgets, rectángulo sucio, cola de operaciones) y
//! [`Frame`](crate::frame::Frame) para los locales de pila (rectángulo y registro de operación pasados por dirección).
pub mod confirm;
pub mod lists;
pub mod owners;
