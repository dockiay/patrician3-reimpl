//! Enumeración del filtro compartida por muestras y streams.

pub trait FilterBackend {
    /// Mantiene la última clave escrita, también cuando devuelve false.
    /// None indica que no se conoce el local que leería el original.
    fn next_filter_attribute(&mut self, provider: u32, cursor: &mut u32, key: &mut Option<u32>) -> bool;
    fn set_sample_processor(&mut self, handle: u32, stage: u32, provider: u32);
    fn set_filter_preference(&mut self, handle: u32, key: u32, value_bits: u32);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterError {
    MissingKey,
}

pub(crate) fn last_key(backend: &mut impl FilterBackend, provider: u32) -> Result<u32, FilterError> {
    let mut cursor = 0;
    let mut key = None;
    while backend.next_filter_attribute(provider, &mut cursor, &mut key) {}
    key.ok_or(FilterError::MissingKey)
}
