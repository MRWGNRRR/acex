// region: LogicalAddress

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalAddress(u16);

impl LogicalAddress {
    pub fn new(val: u16) -> Self {
        Self(val)
    }

    #[must_use]
    #[inline]
    pub fn value(&self) -> u16 {
        self.0
    }
}

// endregion: LogicalAddress

// region: DoipAddress

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoipAddress {
    pub logical: LogicalAddress,
    pub mode: acex_core::AddressMode,
}

impl DoipAddress {
    pub fn new(logical: LogicalAddress, mode: acex_core::AddressMode) -> Self {
        Self { logical, mode }
    }
}

impl acex_core::DiagnosticAddress for DoipAddress {
    fn address_mode(&self) -> acex_core::AddressMode {
        self.mode.clone()
    }
}
