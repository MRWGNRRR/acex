#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;

pub mod ext;

pub mod isotp;

pub mod constants;

// Re-exports
pub use error::{CanError, IsoTpError};
pub use ext::classic::{CanFrameExt, CanFrameMutExt};
pub use ext::fd::{dlc_to_len, len_to_dlc, CanFdFrameExt, CanFdFrameMutExt};
pub use isotp::address::IsoTpAddressingMode;
pub use isotp::pci::{FlowStatus, PciFrame};
pub use isotp::reassembler::{ReassembleResult, Reassembler, ReassemblerConfig};
pub use isotp::segmenter::{SegmentResult, Segmenter, SegmenterConfig};
