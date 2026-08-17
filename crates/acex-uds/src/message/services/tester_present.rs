use crate::UdsError;
use acex_macros::FrameCodec;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TesterPresentRequest {
    pub zero_sub_function: ZeroSubFunction,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TesterPresentResponse {
    pub zero_sub_function: ZeroSubFunction,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZeroSubFunction {
    #[frame(id = 0x00)]
    ZeroSubFunction,
    #[frame(id_pat = "0x01..=0x7F")]
    IsoSaeReserved(u8),
}
