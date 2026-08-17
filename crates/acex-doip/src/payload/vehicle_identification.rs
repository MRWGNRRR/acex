use acex_macros::FrameCodec;
use acex_proto::doip::constants::{DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN};

use crate::error::DoipError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = DoipError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VehicleIdentificationRequest {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = DoipError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VehicleIdentificationRequestEid {
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = DoipError)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VehicleIdentificationRequestVin {
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
}
