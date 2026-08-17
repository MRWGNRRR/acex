use crate::CanId;

// region: CanAddress

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanAddress {
    pub id: CanId,
    pub mode: acex_core::AddressMode,
}

impl CanAddress {
    pub fn new(id: CanId, mode: acex_core::AddressMode) -> Self {
        Self { id, mode }
    }
}

impl acex_core::DiagnosticAddress for CanAddress {
    fn address_mode(&self) -> acex_core::AddressMode {
        self.mode.clone()
    }
}

// endregion: CanAddress
