// region: NRC Error

/// A trait for application error types that can be converted into UDS Negative Response Codes
/// (NRCs).
///
/// The server state machine uses this to construct NRC responses without knowing the application's
/// error type internals. The application defines the mapping via `Into<u8>`.

pub trait NrcError: Into<u8> + core::fmt::Debug {
    // region: Mandatory constructors - one per NRC the server may emit

    /// 0x10 - general reject
    fn general_reject() -> Self;

    /// 0x11 - serviceNotSupported
    fn service_not_supported() -> Self;
    /// 0x12 - subFunctionNotSupported
    fn sub_function_not_supported() -> Self;
    /// 0x13 - incorrectMessageLengthOrInvalidFormat
    fn incorrect_message_length_or_invalid_format() -> Self;

    /// 0x14 - response too long
    fn response_too_long() -> Self;

    /// 0x21 - busy repeat request
    fn busy_repeat_request() -> Self;

    /// 0x22 - conditionsNotCorrect
    fn conditions_not_correct() -> Self;
    /// 0x24 - requestSequenceError
    fn request_sequence_error() -> Self;

    /// 0x25 - no response from subnet component
    fn no_response_from_subnet_component() -> Self;

    /// 0x26 - failure prevents execution of requested action
    fn failure_prevents_execution_of_requested_action() -> Self;

    /// 0x31 - requestOutOfRange
    fn request_out_of_range() -> Self;
    /// 0x33 - securityAccessDenied
    fn security_access_denied() -> Self;
    /// 0x35 - invalidKey
    fn invalid_key() -> Self;
    /// 0x36 - exceededNumberOfAttempts
    fn exceeded_number_of_attempts() -> Self;
    /// 0x37 - requiredTimeDelayNotExpired
    fn required_time_delay_not_expired() -> Self;
    /// 0x70 - uploadDownloadNotAccepted
    fn upload_download_not_accepted() -> Self;
    /// 0x71 - transferDataSuspended
    fn transfer_data_suspended() -> Self;
    /// 0x72 - generalProgrammingFailure
    fn general_programming_failure() -> Self;
    /// 0x73 - wrongBlockSequenceCounter
    fn wrong_block_sequence_counter() -> Self;
    /// 0x78 - requestCorrectlyReceivedResponsePending
    fn response_pending() -> Self;
    /// 0x7E - subFunctionNotSupportedInActiveSession
    fn sub_function_not_supported_in_active_session() -> Self;
    /// 0x7F - serviceNotSupportedInActiveSession
    fn service_not_supported_in_active_session() -> Self;

    /// 0x81 - rpmTooHigh
    fn rpm_too_high() -> Self;

    /// 0x82 - rpmTooLow
    fn rpm_too_low() -> Self;

    /// 0x83 - engineIsRunning
    fn engine_is_running() -> Self;

    /// 0x84 - engineIsNotRunning
    fn engine_is_not_running() -> Self;

    /// 0x85 - engineRunTimeTooLow
    fn engine_run_time_too_low() -> Self;

    /// 0x86 - temperatureTooHigh
    fn temperature_too_high() -> Self;

    /// 0x87 - temperatureTooLow
    fn temperature_too_low() -> Self;

    /// 0x88 - vehicleSpeedTooHigh
    fn vehicle_speed_too_high() -> Self;

    /// 0x89 - vehicleSpeedTooLow
    fn vehicle_speed_too_low() -> Self;

    /// 0x8A - throttlePedalTooHigh
    fn throttle_pedal_too_high() -> Self;

    /// 0x8B - throttlePedalTooLow
    fn throttle_pedal_too_low() -> Self;

    /// 0x8C - transmissionRangeNotInNeutral
    fn transmission_range_not_in_neutral() -> Self;

    /// 0x8D - transmissionRangeNotInGear
    fn transmission_range_not_in_gear() -> Self;

    /// 0x8F - brakeSwitchesNotClosed
    fn brake_switches_not_closed() -> Self;

    /// 0x90 - shifterLevelNotInPark
    fn shifter_level_not_in_park() -> Self;

    /// 0x91 - torqueConverterClutchLocked
    fn torque_converter_clutch_locked() -> Self;

    /// 0x92 - voltageTooHigh
    fn voltage_too_high() -> Self;

    /// 0x93 - voltageTooLow
    fn voltage_too_low() -> Self;
    // endregion: Mandatory constructors
}

// endregion: NRC Error

// region: Builtin NRC

/// A built-in NRC type covering all codes the server may emit.
///
/// Applications that do not need a custom error type may use this directly as `type Error =
/// BuiltinNrc` in the `ServerHandler` impl.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum BuiltinNrc {
    GeneralReject = 0x10,
    ServiceNotSupported = 0x11,
    SubFunctionNotSupported = 0x12,
    IncorrectMessageLengthOrInvalidFormat = 0x13,
    ResponseTooLong = 0x14,
    BusyRepeatRequest = 0x21,
    ConditionsNotCorrect = 0x22,
    RequestSequenceError = 0x24,
    NoResponseFromSubnetComponent = 0x25,
    FailurePreventsExecutionOfRequestedAction = 0x26,
    RequestOutOfRange = 0x31,
    SecurityAccessDenied = 0x33,
    InvalidKey = 0x35,
    ExceededNumberOfAttempts = 0x36,
    RequiredTimeDelayNotExpired = 0x37,
    UploadDownloadNotAccepted = 0x70,
    TransferDataSuspended = 0x71,
    GeneralProgrammingFailure = 0x72,
    WrongBlockSequenceCounter = 0x73,
    ResponsePending = 0x78,
    SubFunctionNotSupportedInActiveSession = 0x7E,
    ServiceNotSupportedInActiveSession = 0x7F,
    RpmTooHigh = 0x81,
    RpmTooLow = 0x82,
    EngineIsRunning = 0x83,
    EngineIsNotRunning = 0x84,
    EngineRunTimeTooLow = 0x85,
    TemperatureTooHigh = 0x86,
    TemperatureTooLow = 0x87,
    VehicleSpeedTooHigh = 0x88,
    VehicleSpeedToLow = 0x89,
    ThrottlePedalTooHigh = 0x8A,
    ThrottlePedalTooLow = 0x8B,
    TransmissionRangeNotInNeutral = 0x8C,
    TransmissionRangeNotInGear = 0x8D,
    BrakeSwitchesNotClosed = 0x8F,
    ShifterLevelNotInPark = 0x90,
    TorqueConverterClutchLocked = 0x91,
    VoltageTooHigh = 0x92,
    VoltageTooLow = 0x93
}

impl From<BuiltinNrc> for u8 {
    fn from(n: BuiltinNrc) -> u8 {
        n as u8
    }
}

impl NrcError for BuiltinNrc {
    fn general_reject() -> Self {
        Self::GeneralReject
    }

    fn service_not_supported() -> Self {
        Self::ServiceNotSupported
    }
    fn sub_function_not_supported() -> Self {
        Self::SubFunctionNotSupported
    }
    fn incorrect_message_length_or_invalid_format() -> Self {
        Self::IncorrectMessageLengthOrInvalidFormat
    }

    fn response_too_long() -> Self {
        Self::ResponseTooLong
    }

    fn busy_repeat_request() -> Self {
        Self::BusyRepeatRequest
    }

    fn conditions_not_correct() -> Self {
        Self::ConditionsNotCorrect
    }
    fn request_sequence_error() -> Self {
        Self::RequestSequenceError
    }

    fn no_response_from_subnet_component() -> Self { Self::NoResponseFromSubnetComponent }

    fn failure_prevents_execution_of_requested_action() -> Self {
        Self::FailurePreventsExecutionOfRequestedAction
    }

    fn request_out_of_range() -> Self {
        Self::RequestOutOfRange
    }
    fn security_access_denied() -> Self {
        Self::SecurityAccessDenied
    }
    fn invalid_key() -> Self {
        Self::InvalidKey
    }
    fn exceeded_number_of_attempts() -> Self {
        Self::ExceededNumberOfAttempts
    }
    fn required_time_delay_not_expired() -> Self {
        Self::RequiredTimeDelayNotExpired
    }
    fn upload_download_not_accepted() -> Self {
        Self::UploadDownloadNotAccepted
    }
    fn transfer_data_suspended() -> Self {
        Self::TransferDataSuspended
    }
    fn general_programming_failure() -> Self {
        Self::GeneralProgrammingFailure
    }
    fn wrong_block_sequence_counter() -> Self {
        Self::WrongBlockSequenceCounter
    }
    fn response_pending() -> Self {
        Self::ResponsePending
    }
    fn sub_function_not_supported_in_active_session() -> Self {
        Self::SubFunctionNotSupportedInActiveSession
    }
    fn service_not_supported_in_active_session() -> Self {
        Self::ServiceNotSupportedInActiveSession
    }

    fn rpm_too_high() -> Self {
        Self::RpmTooHigh
    }

    fn rpm_too_low() -> Self {
        Self::RpmTooLow
    }

    fn engine_is_running() -> Self {
        Self::EngineIsRunning
    }

    fn engine_is_not_running() -> Self {
        Self::EngineIsNotRunning
    }

    fn engine_run_time_too_low() -> Self {
        Self::EngineRunTimeTooLow
    }

    fn temperature_too_high() -> Self {
        Self::TemperatureTooHigh
    }

    fn temperature_too_low() -> Self {
        Self::TemperatureTooLow
    }

    fn vehicle_speed_too_high() -> Self {
        Self::VehicleSpeedTooHigh
    }

    fn vehicle_speed_too_low() -> Self {
        Self::VehicleSpeedToLow
    }

    fn throttle_pedal_too_high() -> Self {
        Self::ThrottlePedalTooHigh
    }

    fn throttle_pedal_too_low() -> Self {
        Self::ThrottlePedalTooLow
    }

    fn transmission_range_not_in_neutral() -> Self {
        Self::TransmissionRangeNotInNeutral
    }

    fn transmission_range_not_in_gear() -> Self {
        Self::TransmissionRangeNotInGear
    }

    fn brake_switches_not_closed() -> Self {
        Self::BrakeSwitchesNotClosed
    }

    fn shifter_level_not_in_park() -> Self {
        Self::ShifterLevelNotInPark
    }

    fn torque_converter_clutch_locked() -> Self {
        Self::TorqueConverterClutchLocked
    }

    fn voltage_too_high() -> Self {
        Self::VoltageTooHigh
    }

    fn voltage_too_low() -> Self {
        Self::VoltageTooLow
    }
}

// endregion: Builtin NRC
