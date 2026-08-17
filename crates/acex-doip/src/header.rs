use acex_macros::FrameCodec;
use acex_proto::doip::constants::{
    DEFAULT_VALUE, DOIP_ALIVE_CHECK_REQUEST, DOIP_ALIVE_CHECK_RESPONSE, DOIP_DIAGNOSTIC_MESSAGE,
    DOIP_DIAGNOSTIC_MESSAGE_ACK, DOIP_DIAGNOSTIC_MESSAGE_NACK, DOIP_ENTITY_STATUS_REQUEST,
    DOIP_ENTITY_STATUS_RESPONSE, DOIP_GENERIC_NACK, DOIP_POWER_INFORMATION_REQUEST,
    DOIP_POWER_INFORMATION_RESPONSE, DOIP_ROUTING_ACTIVATION_REQUEST,
    DOIP_ROUTING_ACTIVATION_RESPONSE, DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,
    DOIP_VEHICLE_IDENTIFICATION_REQ, DOIP_VEHICLE_IDENTIFICATION_REQ_EID,
    DOIP_VEHICLE_IDENTIFICATION_REQ_VIN, ISO13400_2010, ISO13400_2012, ISO13400_2019,
    ISO13400_2019_AMD1, RESERVED_VER,
};

use crate::error::DoipError;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = DoipError)]
pub struct DoipHeader {
    pub protocol_version: ProtocolVersion,
    pub inverse_protocol_version: u8,
    pub payload_type: PayloadType,
    pub payload_length: u32,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = DoipError)]
pub enum ProtocolVersion {
    #[frame(id =  RESERVED_VER)]
    ReservedVer,
    #[frame(id =  ISO13400_2010)]
    Iso13400_2010,
    #[frame(id =  ISO13400_2012)]
    Iso13400_2012,
    #[frame(id =  ISO13400_2019)]
    Iso13400_2019,
    #[frame(id =  ISO13400_2019_AMD1)]
    Iso13400_2019Amd1,
    #[frame(id =  DEFAULT_VALUE)]
    DefaultValue,
}

#[repr(u16)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[frame(error = DoipError)]
pub enum PayloadType {
    #[frame(id =  DOIP_GENERIC_NACK)]
    GenericNack,
    #[frame(id =  DOIP_VEHICLE_IDENTIFICATION_REQ)]
    VehicleIdentificationRequest,
    #[frame(id =  DOIP_VEHICLE_IDENTIFICATION_REQ_EID)]
    VehicleIdentificationRequestEid,
    #[frame(id =  DOIP_VEHICLE_IDENTIFICATION_REQ_VIN)]
    VehicleIdentificationRequestVin,
    #[frame(id =  DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE)]
    VehicleAnnouncementMessage,
    #[frame(id =  DOIP_ROUTING_ACTIVATION_REQUEST)]
    RoutingActivationRequest,
    #[frame(id =  DOIP_ROUTING_ACTIVATION_RESPONSE)]
    RoutingActivationResponse,
    #[frame(id =  DOIP_ALIVE_CHECK_REQUEST)]
    AliveCheckRequest,
    #[frame(id =  DOIP_ALIVE_CHECK_RESPONSE)]
    AliveCheckResponse,
    #[frame(id =  DOIP_ENTITY_STATUS_REQUEST)]
    EntityStatusRequest,
    #[frame(id =  DOIP_ENTITY_STATUS_RESPONSE)]
    EntityStatusResponse,
    #[frame(id =  DOIP_POWER_INFORMATION_REQUEST)]
    PowerInformationRequest,
    #[frame(id =  DOIP_POWER_INFORMATION_RESPONSE)]
    PowerInformationResponse,
    #[frame(id =  DOIP_DIAGNOSTIC_MESSAGE)]
    DiagnosticMessage,
    #[frame(id =  DOIP_DIAGNOSTIC_MESSAGE_ACK)]
    DiagnosticMessageAck,
    #[frame(id =  DOIP_DIAGNOSTIC_MESSAGE_NACK)]
    DiagnosticMessageNack,
}
