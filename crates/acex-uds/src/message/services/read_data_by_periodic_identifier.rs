use crate::UdsError;
use acex_macros::FrameCodec;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadDataByPeriodicIdentifierRequest<'a> {
    pub transmission_mode: TransmissionMode,
    pub periodic_data_identifiers: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadDataByPeriodicIdentifierResponse {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadDataByPeriodicIdentifierResponseData<'a> {
    pub periodic_data_identifier: u8,
    pub data_record: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TransmissionMode {
    #[frame(id_pat = "0x00 | 0x05..=0xFF")]
    IsoSaeReserved(u8),
    #[frame(id = 0x01)]
    SendAtSlowRate,
    #[frame(id = 0x02)]
    SendAtMediumRate,
    #[frame(id = 0x03)]
    SendAtFastRate,
    #[frame(id = 0x04)]
    StopSending,
}
