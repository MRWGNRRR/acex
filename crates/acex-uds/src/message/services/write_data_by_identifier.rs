use crate::{message::DataIdentifier, UdsError};
use acex_macros::FrameCodec;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteDataByIdentifierRequest<'a> {
    pub data_identifier: DataIdentifier,
    pub data_record: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = UdsError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteDataByIdentifierResponse {
    pub data_identifier: DataIdentifier,
}
