use crate::UdsError;
use ace_macros::FrameCodec;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[frame(error = "UdsError")]
#[repr(u8)]
pub enum ServiceIdentifier {
    #[frame(
        id_pat = "0x00 | 0x3F | 0x40 | 0x80..=0x83 | 0x89..=0xB9 | 0xBF..=0xC2 | 0xC9..=0xF9 | 0xFF"
    )]
    NotApplicable(u8),

    #[frame(id_pat = "0x01..=0x0F")]
    EmissionsSpecificServiceRequest(u8),

    #[frame(id_pat = "0x10..=0x3E", decode_inner)]
    UdsServiceRequest(UdsServiceRequest),

    #[frame(id_pat = "0x41..=0x4F")]
    EmissionsSpecificServicePositiveResponse(u8),

    #[frame(id_pat = "0x50..=0x7E", decode_inner)]
    UdsServiceResponse(UdsServiceResponse),

    #[frame(id = "0x7F")]
    NegativeResponse,

    #[frame(id_pat = "0x84..=0x88")]
    ServiceRequests(u8),

    #[frame(id_pat = "0xBA..=0xBE")]
    SystemSupplierServiceRequests(u8),

    #[frame(id_pat = "0xC3..=0xC8")]
    ServiceRequestsPositiveResponse(u8),

    #[frame(id_pat = "0xFA..=0xFE")]
    SystemSupplierServiceRequestsPositiveResponse(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[frame(error = "UdsError")]
#[repr(u8)]
pub enum UdsServiceResponse {
    #[frame(id = 0x50)]
    DiagnosticSessionControl = 0x50,
    #[frame(id = 0x51)]
    EcuReset = 0x51,
    #[frame(id = 0x67)]
    SecurityAccess = 0x67,
    #[frame(id = 0x68)]
    CommunicationControl = 0x68,
    #[frame(id = 0x69)]
    Authentication = 0x69,
    #[frame(id = 0x7e)]
    TesterPresent = 0x7e,
    #[frame(id = 0xc5)]
    ControlDtcSetting = 0xc5,
    #[frame(id = 0xc6)]
    ResponseOnEvent = 0xc6,
    #[frame(id = 0xc7)]
    LinkControl = 0xc7,
    #[frame(id = 0x62)]
    ReadDataByIdentifier = 0x62,
    #[frame(id = 0x63)]
    ReadMemoryByAddress = 0x63,
    #[frame(id = 0x64)]
    ReadScalingDataByIdentifier = 0x64,
    #[frame(id = 0x6a)]
    ReadDataByPeriodicIdentifier = 0x6a,
    #[frame(id = 0x6c)]
    DynamicallyDefineDataIdentifier = 0x6c,
    #[frame(id = 0x6e)]
    WriteDataByIdentifier = 0x6e,
    #[frame(id = 0x7d)]
    WriteMemoryByAddress = 0x7d,
    #[frame(id = 0x54)]
    ClearDiagnosticInformation = 0x54,
    #[frame(id = 0x59)]
    ReadDtcInformation = 0x59,
    #[frame(id = 0x6f)]
    InputOutputControlByIdentifier = 0x6f,
    #[frame(id = 0x71)]
    RoutineControl = 0x71,
    #[frame(id = 0x74)]
    RequestDownload = 0x74,
    #[frame(id = 0x75)]
    RequestUpload = 0x75,
    #[frame(id = 0x76)]
    TransferData = 0x76,
    #[frame(id = 0x77)]
    RequestTransferExit = 0x77,
    #[frame(id = 0x78)]
    RequestFileTransfer = 0x78,
    #[frame(id = 0xc4)]
    SecuredDataTransmission = 0xc4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, FrameCodec)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[frame(error = "UdsError")]
#[repr(u8)]
pub enum UdsServiceRequest {
    #[frame(id = 0x10)]
    DiagnosticSessionControl = 0x10,
    #[frame(id = 0x11)]
    EcuReset = 0x11,
    #[frame(id = 0x27)]
    SecurityAccess = 0x27,
    #[frame(id = 0x28)]
    CommunicationControl = 0x28,
    #[frame(id = 0x29)]
    Authentication = 0x29,
    #[frame(id = 0x3e)]
    TesterPresent = 0x3e,
    #[frame(id = 0x85)]
    ControlDtcSetting = 0x85,
    #[frame(id = 0x86)]
    ResponseOnEvent = 0x86,
    #[frame(id = 0x87)]
    LinkControl = 0x87,
    #[frame(id = 0x22)]
    ReadDataByIdentifier = 0x22,
    #[frame(id = 0x23)]
    ReadMemoryByAddress = 0x23,
    #[frame(id = 0x24)]
    ReadScalingDataByIdentifier = 0x24,
    #[frame(id = 0x2a)]
    ReadDataByPeriodicIdentifier = 0x2a,
    #[frame(id = 0x2c)]
    DynamicallyDefineDataIdentifier = 0x2c,
    #[frame(id = 0x2e)]
    WriteDataByIdentifier = 0x2e,
    #[frame(id = 0x3d)]
    WriteMemoryByAddress = 0x3d,
    #[frame(id = 0x14)]
    ClearDiagnosticInformation = 0x14,
    #[frame(id = 0x19)]
    ReadDTCInformation = 0x19,
    #[frame(id = 0x2f)]
    InputOutputControlByIdentifier = 0x2f,
    #[frame(id = 0x31)]
    RoutineControl = 0x31,
    #[frame(id = 0x34)]
    RequestDownload = 0x34,
    #[frame(id = 0x35)]
    RequestUpload = 0x35,
    #[frame(id = 0x36)]
    TransferData = 0x36,
    #[frame(id = 0x37)]
    RequestTransferExit = 0x37,
    #[frame(id = 0x38)]
    RequestFileTransfer = 0x38,
    #[frame(id = 0x84)]
    SecuredDataTransmission = 0x84,
}

impl ServiceIdentifier {
    pub fn discriminant(&self) -> u8 {
        match self {
            ServiceIdentifier::UdsServiceRequest(s) => *s as u8,
            ServiceIdentifier::UdsServiceResponse(s) => *s as u8,
            ServiceIdentifier::NegativeResponse => 0x7F,
            ServiceIdentifier::NotApplicable(v)
            | ServiceIdentifier::EmissionsSpecificServiceRequest(v)
            | ServiceIdentifier::EmissionsSpecificServicePositiveResponse(v)
            | ServiceIdentifier::ServiceRequests(v)
            | ServiceIdentifier::SystemSupplierServiceRequests(v)
            | ServiceIdentifier::ServiceRequestsPositiveResponse(v)
            | ServiceIdentifier::SystemSupplierServiceRequestsPositiveResponse(v) => *v,
        }
    }
    /// Returns `true` if this service defines a sub-function byte at offset 1.
    ///
    /// Per ISO 14229, the following services carry a sub-function byte.
    /// Services excluded are those whose second byte is a data parameter
    /// rather than a sub-function - for example `ReadDataByIdentifier`
    /// uses a 2-byte DID, not a sub-function.
    #[must_use]
    pub fn has_sub_function(&self) -> bool {
        matches!(
            self,
            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::DiagnosticSessionControl)
                | ServiceIdentifier::UdsServiceResponse(
                    UdsServiceResponse::DiagnosticSessionControl
                )
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::EcuReset)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::EcuReset)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::SecurityAccess)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::SecurityAccess)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::CommunicationControl)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::CommunicationControl)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::Authentication)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::Authentication)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::TesterPresent)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::TesterPresent)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::ControlDtcSetting)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::ControlDtcSetting)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::ResponseOnEvent)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::ResponseOnEvent)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::LinkControl)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::LinkControl)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::ReadDTCInformation)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::ReadDtcInformation)
                | ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::RoutineControl)
                | ServiceIdentifier::UdsServiceResponse(UdsServiceResponse::RoutineControl)
        )
    }
}
