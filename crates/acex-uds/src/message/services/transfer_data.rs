use crate::UdsError;
use acex_macros::FrameCodec;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TransferDataRequest<'a> {
    pub block_sequence_counter: u8,
    pub transfer_request_parameter_record: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TransferDataResponse<'a> {
    pub block_sequence_counter: u8,
    pub transfer_response_parameter_record: &'a [u8],
}
