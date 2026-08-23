// region: Imports

use acex_core::Vec;
use acex_proto::uds::UdsFrame;
use acex_sim::clock::{Duration, Instant};
use acex_sim::io::NodeAddress;
use acex_uds::ext::UdsFrameExt;
use acex_uds::message::service::UdsServiceRequest;
use acex_uds::message::{DiagnosticSessionType, ServiceIdentifier};

use crate::config::{periodic, ServerConfig, SessionConfig};
use crate::handler::ServerHandler;
use crate::nrc::NrcError;
use crate::security_provider::SecurityProvider;

// endregion: Imports

// region: ServerError

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ServerError<E: NrcError> {
    Handler(E),
    Codec(acex_uds::error::UdsError),
    OutboxFull,
}

// endregion: ServerError

// region: SessionState

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SessionState {
    session_type: u8,
    last_rx: Instant,
    security_level: u8,
}

impl PartialEq for SessionState {
    fn eq(&self, other: &Self) -> bool {
        self.session_type == other.session_type
            && self.security_level == other.security_level
    }
}

impl SessionState {
    fn new(session_type: u8) -> Self {
        Self {
            session_type,
            last_rx: Instant::ZERO,
            security_level: 0,
        }
    }

    pub const fn is_default(&self) -> bool {
        self.session_type == 0x01
    }

    pub const fn is_programming(&self) -> bool {
        self.session_type == 0x02
    }

    pub const fn is_extended(&self) -> bool {
        self.session_type == 0x03
    }

    pub const fn get_session_type(&self) -> u8 {
        self.session_type
    }

    pub const fn get_last_rx(&self) -> Instant {
        self.last_rx
    }

    pub const fn get_security_level(&self) -> u8 {
        self.security_level
    }
}

// endregion: SessionState

// region: SecurityState

#[derive(Debug, Clone)]
#[cfg_attr(all(feature = "defmt", not(feature = "alloc")), derive(defmt::Format))]
struct SecurityState<const MAX_SEED: usize, const MAX_SECURITY_LEVELS: usize> {
    pending_seed: Vec<u8, MAX_SEED>,
    pending_level: u8,
    failed_attempts: Vec<(u8, u8), MAX_SECURITY_LEVELS>,
    lockout_until: Vec<(u8, Instant), MAX_SECURITY_LEVELS>,
}

impl<
    const MAX_SEED: usize,
    const MAX_SECURITY_LEVELS: usize
> SecurityState<
    MAX_SEED,
    MAX_SECURITY_LEVELS
> {
    fn new() -> Self {
        Self {
            pending_seed: Vec::new(),
            pending_level: 0,
            failed_attempts: Vec::new(),
            lockout_until: Vec::new(),
        }
    }

    fn is_locked(&self, level: u8, now: Instant) -> bool {
        self.lockout_until
            .iter()
            .find(|(l, _)| *l == level)
            .map(|(_, until)| now < *until)
            .unwrap_or(false)
    }

    fn failed_count(&self, level: u8) -> u8 {
        self.failed_attempts
            .iter()
            .find(|(l, _)| *l == level)
            .map(|(_, c)| *c)
            .unwrap_or(0)
    }

    fn increment_failed(&mut self, level: u8) {
        if let Some(e) = self.failed_attempts.iter_mut().find(|(l, _)| *l == level) {
            e.1 = e.1.saturating_add(1);
        } else {
            #[cfg(all(feature = "defmt", not(feature = "alloc")))]
            defmt::unwrap!(self.failed_attempts.push((level, 1))); // unlikely

            #[cfg(feature = "alloc")]
            self.failed_attempts.push((level, 1));
        }
    }

    fn reset_failed(&mut self, level: u8) {
        if let Some(e) = self.failed_attempts.iter_mut().find(|(l, _)| *l == level) {
            e.1 = 0;
        }
    }

    fn set_lockout(&mut self, level: u8, until: Instant) {
        if let Some(e) = self.lockout_until.iter_mut().find(|(l, _)| *l == level) {
            e.1 = until;
        } else {
            #[cfg(all(feature = "defmt", not(feature = "alloc")))]
            defmt::unwrap!(self.lockout_until.push((level, until))); // unlikely

            #[cfg(feature = "alloc")]
            self.lockout_until.push((level, until));
        }
    }

    fn clear_pending(&mut self) {
        self.pending_seed.clear();
        self.pending_level = 0;
    }

    fn reset(&mut self) {
        self.lockout_until.clear();
        self.pending_seed.clear();
        self.failed_attempts.clear();
        self.pending_level = 0 ;
    }
}

// endregion: SecurityState

// region: PeriodicEntry / PeriodicState

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct PeriodicEntry {
    did: u16,
    interval: Duration,
    next_tx: Instant,
    client: NodeAddress,
}

#[derive(Debug)]
#[cfg_attr(all(feature = "defmt", not(feature = "alloc")), derive(defmt::Format))]
struct PeriodicState<const MAX_PERIODIC: usize> {
    entries: Vec<PeriodicEntry, MAX_PERIODIC>,
}

impl<const MAX_PERIODIC: usize> PeriodicState<MAX_PERIODIC> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn register(&mut self, did: u16, interval: Duration, client: NodeAddress, now: Instant) {
        if let Some(e) = self
            .entries
            .iter_mut()
            .find(|e| e.did == did && e.client == client)
        {
            e.interval = interval;
            e.next_tx = now + interval;
            return;
        }

        #[cfg(feature = "defmt")]
        defmt::unwrap!(self.entries.push(PeriodicEntry {
            did,
            interval,
            next_tx: now + interval,
            client,
        }));

        #[cfg(not(feature = "defmt"))]
        let _ = self.entries.push(PeriodicEntry {
            did,
            interval,
            next_tx: now + interval,
            client,
        });
    }

    fn cancel(&mut self, did: u16, client: &NodeAddress) {
        self.entries
            .retain(|e| !(e.did == did && &e.client == client));
    }

    fn collect_due(&self, now: Instant, out: &mut Vec<(u16, NodeAddress), MAX_PERIODIC>) {
        for e in self.entries.iter().filter(|e| now >= e.next_tx) {
            #[cfg(feature = "defmt")]
            defmt::unwrap!(out.push((e.did, e.client.clone())));

            #[cfg(not(feature = "defmt"))]
            let _ = out.push((e.did, e.client.clone()));
        }
    }

    fn advance(&mut self, did: u16, client: &NodeAddress, now: Instant) {
        if let Some(e) = self
            .entries
            .iter_mut()
            .find(|e| e.did == did && &e.client == client)
        {
            e.next_tx = now + e.interval;
        }
    }
}

// endregion: PeriodicEntry / PeriodicState

// region: UdsServer

/// Stateful UDS ECU server state machine.
///
/// Receives raw UDS frames via [`handle`], uses [`UdsFrameExt`] for
/// protocol-level decisions (SID dispatch, suppress bit, sub-function),
/// and decodes typed messages only where structured field access is needed.
///
/// All timing is driven by [`tick`] - no blocking, no hardware timers,
/// no OS calls. Suitable for direct use as a `SimNode` in `ace-sim`.
#[derive(Debug)]
#[cfg_attr(all(feature = "defmt", not(feature = "alloc")), derive(defmt::Format))]
pub struct UdsServer<
    const MAX_FRAME: usize,
    const MAX_OUTBOX: usize,
    const MAX_SESSIONS: usize,
    const MAX_SERVICES: usize,
    const MAX_DIDS: usize,
    const MAX_SECURITY_LEVELS: usize,
    const DEFAULT_S3: u64,
    const DEFAULT_P2: u64,
    const DEFAULT_P2_EXT: u64,
    const DEFAULT_LOCKOUT: u64,
    const DEFAULT_MAX_SECURITY_ATTEMPTS: u8,
    const MAX_SEED: usize,
    const MAX_PERIODIC: usize,
    H,
    S,
> where
    H: ServerHandler<S>,
    S: SecurityProvider,
{
    config: ServerConfig<MAX_SESSIONS, MAX_SERVICES, MAX_DIDS, MAX_SECURITY_LEVELS>,
    handler: H,
    security_provider: S,
    address: NodeAddress,
    session: SessionState,
    security: SecurityState<MAX_SEED, MAX_SECURITY_LEVELS>,
    periodic: PeriodicState<MAX_PERIODIC>,
    outbox: Vec<(NodeAddress, Vec<u8, MAX_FRAME>), MAX_OUTBOX>,

    /// Whether pending requests should be dropped.
    drop_requests: bool,
}

impl<
        const MAX_FRAME: usize,
        const MAX_OUTBOX: usize,
        const MAX_SESSIONS: usize,
        const MAX_SERVICES: usize,
        const MAX_DIDS: usize,
        const MAX_SECURITY_LEVELS: usize,
        const DEFAULT_S3: u64,
        const DEFAULT_P2: u64,
        const DEFAULT_P2_EXT: u64,
        const DEFAULT_LOCKOUT: u64,
        const DEFAULT_MAX_SECURITY_ATTEMPTS: u8,
        const MAX_SEED: usize,
        const MAX_PERIODIC: usize,
        H,
        S,
    >
    UdsServer<
        MAX_FRAME,
        MAX_OUTBOX,
        MAX_SESSIONS,
        MAX_SERVICES,
        MAX_DIDS,
        MAX_SECURITY_LEVELS,
        DEFAULT_S3,
        DEFAULT_P2,
        DEFAULT_P2_EXT,
        DEFAULT_LOCKOUT,
        DEFAULT_MAX_SECURITY_ATTEMPTS,
        MAX_SEED,
        MAX_PERIODIC,
        H,
        S,
    >
where
    H: ServerHandler<S>,
    S: SecurityProvider,
{
    pub fn new(
        config: ServerConfig<MAX_SESSIONS, MAX_SERVICES, MAX_DIDS, MAX_SECURITY_LEVELS>,
        handler: H,
        security_provider: S,
        address: NodeAddress,
    ) -> Self {
        Self {
            session: SessionState::new(config.default_session_type.into()),
            config,
            handler,
            security_provider,
            address,
            security: SecurityState::new(),
            periodic: PeriodicState::new(),
            outbox: Vec::new(),
            drop_requests: false
        }
    }

    // region: SimNode surface

    pub fn set_drop_requests(&mut self, status: bool) {
        self.drop_requests = status;
    }

    pub fn drop_requests(&self) -> bool {
        self.drop_requests
    }

    pub fn address(&self) -> &NodeAddress {
        &self.address
    }

    pub fn set_session_type(&mut self, session_type: u8) {
        self.session.session_type = session_type;
    }

    pub fn session_type(&self) -> u8 {
        self.session.session_type
    }

    pub fn set_security_level(&mut self, level: u8) {
        self.session.security_level = level;
    }

    pub fn security_level(&self) -> u8 {
        self.session.security_level
    }

    pub fn reset_security_state(&mut self) {
        self.security.reset();
    }

    /// Receives a raw UDS frame from `src`.
    ///
    /// Wraps the bytes in a [`UdsFrame`] and uses [`UdsFrameExt`] for
    /// protocol-level decisions before dispatching to service handlers.
    /// Typed message decode via `to_message()` is deferred to individual
    /// handlers that need structured field access.
    pub fn handle(
        &mut self,
        src: &NodeAddress,
        data: &[u8],
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        if self.drop_requests {
            #[cfg(feature = "defmt")]
            defmt::info!("uds request dropped: {=[u8]}", data);

            return Ok(());
        }

        self.session.last_rx = now;

        let frame = UdsFrame::from_slice(data);

        // Validate frame has at minimum a SID byte
        if let Err(e) = frame.validate() {
            return Err(ServerError::Codec(e));
        }

        let sid = match frame.service_identifier() {
            Some(s) => s,
            None => return self.nrc_raw(src, 0x00, H::Error::service_not_supported().into(), now),
        };

        // Guard: service must be supported in the active session
        let sid_byte = sid.discriminant();
        if let Err(nrc) = self.guard_service(sid_byte) {
            return self.nrc_raw(src, sid_byte, nrc, now);
        }

        // Suppress bit - extracted here at the frame level before any
        // typed decode. Each handler receives this as a plain bool.
        let suppressed = frame.is_suppressed();

        match sid {
            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::TesterPresent) => {
                self.on_tester_present(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::DiagnosticSessionControl) => {
                self.on_session_control(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::EcuReset) => {
                self.on_ecu_reset(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::SecurityAccess) => {
                self.on_security_access(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::ReadDataByIdentifier) => {
                self.on_read_did(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::WriteDataByIdentifier) => {
                self.on_write_did(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(
                UdsServiceRequest::ReadDataByPeriodicIdentifier,
            ) => self.on_periodic_did(src, &frame, suppressed, now),

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::RoutineControl) => {
                self.on_routine_control(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::CommunicationControl) => {
                self.on_communication_control(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(
                UdsServiceRequest::InputOutputControlByIdentifier,
            ) => self.on_io_control(src, &frame, suppressed, now),

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::RequestDownload) => {
                self.on_request_download(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::TransferData) => {
                self.on_transfer_data(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::RequestTransferExit) => {
                self.on_transfer_exit(src, &frame, suppressed, now)
            }

            ServiceIdentifier::UdsServiceRequest(UdsServiceRequest::RequestFileTransfer) => {
                self.on_file_transfer(src, &frame, suppressed, now)
            }

            _ => self.nrc_raw(src, sid_byte, H::Error::service_not_supported().into(), now),
        }
    }

    /// Advances internal timers - S3 watchdog and periodic DID scheduling.
    pub fn tick(&mut self, now: Instant) -> Result<(), ServerError<H::Error>> {
        self.check_s3(now);
        self.dispatch_periodic(now)
    }

    /// Drains pending outbound frames into `out`.
    pub fn drain_outbox(
        &mut self,
        out: &mut Vec<(NodeAddress, Vec<u8, MAX_FRAME>), MAX_OUTBOX>,
    ) -> usize {
        let n = self.outbox.len();
        for item in self.outbox.drain(..) {
            #[cfg(feature = "defmt")]
            defmt::unwrap!(out.push(item)); // unlikely

            #[cfg(not(feature = "defmt"))]
            let _ = out.push(item);
        }
        n
    }

    // endregion: SimNode surface

    // region: Session helpers

    fn current_session(&self) -> Option<&SessionConfig> {
        self.config.find_session(self.session.session_type)
    }

    fn check_s3(&mut self, now: Instant) {
        if self.session.is_default() {
            return;
        }

        let s3 = self
            .current_session()
            .map(|s| s.s3_timeout)
            .unwrap_or(Duration::from_millis(DEFAULT_S3));

        if let Some(elapsed) = now.checked_duration_since(self.session.last_rx) && s3.as_micros() > 0 {
            if elapsed > s3 {
                self.session.session_type = self.config.default_session_type.into();
                self.session.security_level = 0;
                self.security.clear_pending();
            }
        }
    }

    fn guard_service(&self, sid: u8) -> Result<(), u8> {
        if !self.config.service_allowed(sid, self.session.session_type) {
            return Err(H::Error::service_not_supported_in_active_session().into());
        }
        Ok(())
    }

    fn guard_security(&self, required: u8) -> Result<(), u8> {
        if required > 0 && self.session.security_level < required {
            return Err(H::Error::security_access_denied().into());
        }
        Ok(())
    }

    // endregion: Session helpers

    // region: Response helpers

    fn enqueue(
        &mut self,
        dst: NodeAddress,
        frame: Vec<u8, MAX_FRAME>,
    ) -> Result<(), ServerError<H::Error>> {
        if self.outbox.len() >= MAX_OUTBOX {
            Err(ServerError::OutboxFull)
        } else {
            #[cfg(feature = "defmt")]
            defmt::unwrap!(self.outbox.push((dst, frame))); // unlikely

            #[cfg(not(feature = "defmt"))]
            self.outbox.push((dst, frame));

            Ok(())
        }
    }

    fn pos(
        &mut self,
        dst: &NodeAddress,
        request_sid: u8,
        payload: &[u8],
        _now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let mut frame: Vec<u8, MAX_FRAME> = Vec::new();

        #[cfg(feature = "defmt")]
        {
            defmt::unwrap!(frame.push(request_sid | 0x40));
            defmt::unwrap!(frame.extend_from_slice(payload));
        }

        #[cfg(not(feature = "defmt"))]
        {
            let _ = frame.push(request_sid | 0x40);
            let _ = frame.extend_from_slice(payload);
        }


        self.enqueue(dst.clone(), frame)
    }

    fn nrc(
        &mut self,
        dst: &NodeAddress,
        request_sid: u8,
        error: H::Error,
        _now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let nrc_byte: u8 = error.into();
        self.nrc_raw(dst, request_sid, nrc_byte, _now)
    }

    fn nrc_raw(
        &mut self,
        dst: &NodeAddress,
        request_sid: u8,
        nrc_byte: u8,
        _now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let mut frame: Vec<u8, MAX_FRAME> = Vec::new();

        #[cfg(feature = "defmt")]
        {
            defmt::unwrap!(frame.push(0x7F));
            defmt::unwrap!(frame.push(request_sid));
            defmt::unwrap!(frame.push(nrc_byte));
        }

        #[cfg(not(feature = "defmt"))]
        {
            let _ = frame.push(0x7F);
            let _ = frame.push(request_sid);
            let _ = frame.push(nrc_byte);
        }

        self.enqueue(dst.clone(), frame)
    }

    // endregion: Response helpers

    // region: Service handlers
    //
    // Each handler receives:
    //   - `frame` - the raw UdsFrame for sub-function/payload access
    //   - `suppressed` - suppress bit already extracted at dispatch
    //
    // Typed decode via `frame.to_message()` is used only where structured
    // field access is needed. Services that only need the sub-function value
    // and a payload slice never allocate a typed message.

    fn on_tester_present(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        // TesterPresent: sub-function is always 0x00.
        // The only meaningful information is the suppress bit.
        if suppressed {
            return Ok(());
        }
        // Echo sub-function value (suppress bit cleared) in response.
        let sf = frame.sub_function_value().unwrap_or(0x00);
        self.pos(src, 0x3E, &[sf], now)
    }

    fn on_session_control(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        // Sub-function value IS the session type byte.
        let session_type = match frame.sub_function_value() {
            Some(v) => v,
            None => {
                return self.nrc(
                    src,
                    0x10,
                    H::Error::incorrect_message_length_or_invalid_format(),
                    now,
                )
            }
        };

        if self.config.find_session(session_type).is_none() {
            return self.nrc(src, 0x10, H::Error::sub_function_not_supported(), now);
        }

        let mut ctx = self.create_request_context();
        let result = self.handler.session_control(&mut ctx, session_type);
        self.handle_request_context(ctx);

        if let Err(err) = result {
            return self.nrc(src, 0x10, err, now);
        }
        
        self.session.session_type = session_type;
        self.session.security_level = 0;
        self.session.last_rx = now;
        self.security.clear_pending();
        
        if suppressed {
            return Ok(());
        }

        let (p2_ms, p2_ext_ms) = self
            .config
            .find_session(session_type)
            .map(|s| {
                (
                    s.p2_timeout.as_millis(),
                    s.p2_extended_timeout.as_millis() / 10,
                )
            })
            .unwrap_or((DEFAULT_P2, DEFAULT_P2_EXT));

        let payload = [
            session_type,
            (p2_ms >> 8) as u8,
            p2_ms as u8,
            (p2_ext_ms >> 8) as u8,
            p2_ext_ms as u8,
        ];
        self.pos(src, 0x10, &payload, now)
    }

    fn on_ecu_reset(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        // Sub-function value IS the reset type byte.
        let reset_type = match frame.sub_function_value() {
            Some(v) => v,
            None => {
                return self.nrc(
                    src,
                    0x11,
                    H::Error::incorrect_message_length_or_invalid_format(),
                    now,
                )
            }
        };

        let mut ctx = self.create_request_context();

        let result = self.handler
            .ecu_reset(&mut ctx, reset_type);

        self.handle_request_context(ctx);

        if let Err(err) = result {
            return self.nrc(src, 0x11, err, now);
        }

        if !suppressed {
            self.pos(src, 0x11, &[reset_type], now)?;
        }

        Ok(())
    }

    fn on_security_access(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool, // SecurityAccess has no suppress bit
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        // Access type byte: odd = RequestSeed, even = SendKey.
        let access_type = match frame.sub_function_value() {
            Some(v) => v,
            None => {
                return self.nrc(
                    src,
                    0x27,
                    H::Error::incorrect_message_length_or_invalid_format(),
                    now,
                )
            }
        };

        let is_request_seed = access_type % 2 != 0;

        if is_request_seed {
            let level = access_type;

            if self.security.is_locked(level, now) {
                return self.nrc(src, 0x27, H::Error::required_time_delay_not_expired(), now);
            }

            let Some(config) = self.config.find_security_level(level) else {
                return self.nrc(src, 0x27, H::Error::sub_function_not_supported(), now);
            };

            let mut frame: Vec<u8, MAX_FRAME> = Vec::new();

            if self.security.pending_level == level
                && !self.security.pending_seed.is_empty()
                && self.security.pending_seed.len() == config.seed_length
            {
                frame.push(0x27 | 0x40).unwrap();
                frame.push(level).unwrap();
                frame.extend_from_slice(self.security.pending_seed.as_slice()).unwrap();
                return self.enqueue(src.clone(), frame);
            }

            let mut ctx = self.create_request_context();
            let result = self.handler.security_access(&mut ctx, level, &[]);
            self.handle_request_context(ctx);

            if let Err(err) = result {
                return self.nrc_raw(src, 0x27, err.into(), now);
            }

            let mut seed_buf = [0u8; MAX_SEED];
            let result = self
                .security_provider
                .generate_seed(level, &mut seed_buf)
                .map_err(|_| H::Error::conditions_not_correct());

            let seed_len = match result {
                Ok(v) => v,
                Err(err) => {
                    return self.nrc_raw(src, 0x27, err.into(), now);
                }
            };
            
            self.security.pending_seed.clear();
            self.security.pending_level = level;

            #[cfg(feature = "defmt")]
            {
                defmt::unwrap!(self.security.pending_seed.extend_from_slice(&seed_buf[..seed_len]));
                defmt::unwrap!(frame.push(0x27 | 0x40));
                defmt::unwrap!(frame.push(level));
                defmt::unwrap!(frame.extend_from_slice(&seed_buf[..seed_len]));
            }

            #[cfg(not(feature = "defmt"))]
            {
                let _ = self.security.pending_seed.extend_from_slice(&seed_buf[..seed_len]);
                let _ = frame.push(0x27 | 0x40);
                let _ = frame.push(level);
                let _ = frame.extend_from_slice(&seed_buf[..seed_len]);
            }

            self.enqueue(src.clone(), frame)
        } else {
            // SendKey - key bytes are the payload after the sub-function byte.
            let level = access_type - 1; // RequestSeed level

            if self.security.is_locked(level, now) {
                return self.nrc(src, 0x27, H::Error::required_time_delay_not_expired(), now);
            }
            if self.security.pending_level != level || self.security.pending_seed.is_empty() {
                return self.nrc(src, 0x27, H::Error::request_sequence_error(), now);
            }

            // Key bytes are the payload after the access_type byte.
            // frame.payload() is everything after SID - key starts at payload[1].
            let key = frame.payload().get(1..).unwrap_or(&[]);

            let mut ctx = self.create_request_context();
            let result = self.handler.security_access(&mut ctx, level, key);
            self.handle_request_context(ctx);

            if let Err(err) = result {
                return self.nrc_raw(src, 0x27, err.into(), now);
            }

            let level_cfg = self.config.find_security_level(level);
            let max_attempts = level_cfg
                .map(|l| l.max_attempts)
                .flatten()
                .unwrap_or(DEFAULT_MAX_SECURITY_ATTEMPTS);
            let lockout_dur = level_cfg
                .map(|l| l.lockout_duration)
                .flatten()
                .unwrap_or(Duration::from_millis(DEFAULT_LOCKOUT));

            let seed = self.security.pending_seed.clone();

            match self.security_provider.validate_key(level, &seed, key) {
                Ok(()) => {
                    self.security.reset_failed(level);
                    self.security.clear_pending();
                    self.session.security_level = level;
                    self.pos(src, 0x27, &[access_type], now)
                }
                Err(_) => {
                    self.security.increment_failed(level);
                    if self.security.failed_count(level) >= max_attempts {
                        self.security.set_lockout(level, now + lockout_dur);
                        return self.nrc(src, 0x27, H::Error::exceeded_number_of_attempts(), now);
                    }
                    self.nrc(src, 0x27, H::Error::invalid_key(), now)
                }
            }
        }
    }

    fn on_read_did(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool, // ReadDataByIdentifier has no sub-function
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        // Payload is pairs of DID bytes: [DID_high, DID_low, DID_high, DID_low, ...]
        let payload = frame.payload();
        if payload.len() < 2
            || (!self.config.allow_read_many_dids && payload.len() != 2)
            || payload.len() % 2 != 0
        {
            return self.nrc(
                src,
                0x22,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let mut resp: Vec<u8, MAX_FRAME> = Vec::new();

        for chunk in payload.chunks_exact(2) {
            let did = u16::from_be_bytes([chunk[0], chunk[1]]);

            if !self.config.did_readable(did, self.session.session_type) {
                return self.nrc(src, 0x22, H::Error::request_out_of_range(), now);
            }
            let required_sec = self
                .config
                .find_did(did)
                .map(|d| d.security_level)
                .unwrap_or(0);
            if let Err(nrc) = self.guard_security(required_sec) {
                return self.nrc_raw(src, 0x22, nrc, now);
            }

            #[cfg(feature = "defmt")]
            {
                defmt::unwrap!(resp.push(chunk[0]));
                defmt::unwrap!(resp.push(chunk[1]));
            }

            #[cfg(not(feature = "defmt"))]
            {
                let _ = resp.push(chunk[0]);
                let _ = resp.push(chunk[1]);
            }

            let mut ctx = self.create_request_context();

            let mut data_buf = [0u8; MAX_FRAME];
            let result = self
                .handler
                .read_did(&mut ctx, did, &mut data_buf);

            self.handle_request_context(ctx);

            let len = match result {
                Ok(v) => v,
                Err(err) => {
                    return self.nrc_raw(src, 0x22, err.into(), now);
                }
            };

            #[cfg(feature = "defmt")]
            defmt::unwrap!(resp.extend_from_slice(&data_buf[..len]));

            #[cfg(not(feature = "defmt"))]
            let _ = resp.extend_from_slice(&data_buf[..len]);
        }

        self.pos(src, 0x22, &resp, now)
    }

    fn on_write_did(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let payload = frame.payload();
        if payload.len() < 3 {
            return self.nrc(
                src,
                0x2E,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let did = u16::from_be_bytes([payload[0], payload[1]]);
        let data_rec = &payload[2..];

        if !self.config.did_writable(did, self.session.session_type) {
            return self.nrc(src, 0x2E, H::Error::request_out_of_range(), now);
        }
        let required_sec = self
            .config
            .find_did(did)
            .map(|d| d.security_level)
            .unwrap_or(0);
        if let Err(nrc) = self.guard_security(required_sec) {
            return self.nrc_raw(src, 0x2E, nrc, now);
        }

        let mut ctx = self.create_request_context();

        let result = self.handler
            .write_did(&mut ctx, did, data_rec);

        self.handle_request_context(ctx);

        if let Err(err) = result {
            return self.nrc_raw(src, 0x2E, err.into(), now);
        }

        self.pos(src, 0x2E, &payload[..2], now)
    }

    fn on_periodic_did(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let payload = frame.payload();
        if payload.is_empty() {
            return self.nrc(
                src,
                0x2A,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        // Byte 0 is transmission mode, remaining bytes are periodic DID identifiers.
        let mode = payload[0];
        let periodic_ids = payload.get(1..).unwrap_or(&[]);

        let mut ctx = self.create_request_context();
        let result = self.handler.periodic_did(&mut ctx, mode, periodic_ids);
        self.handle_request_context(ctx);

        if let Err(err) = result {
            return self.nrc_raw(src, 0x2A, err.into(), now);
        }

        match mode {
            // stopSending
            0x04 => {
                for &id in periodic_ids {
                    self.periodic.cancel(0xF200u16 | id as u16, src);
                }
                self.pos(src, 0x2A, &[mode], now)
            }
            0x01 | 0x02 | 0x03 => {
                let requested_interval = match mode {
                    0x01 => periodic::SLOW,
                    0x02 => periodic::MEDIUM,
                    _ => periodic::FAST,
                };

                for &id in periodic_ids {
                    let did = 0xF200u16 | id as u16;

                    if !self.config.did_readable(did, self.session.session_type) {
                        return self.nrc(src, 0x2A, H::Error::request_out_of_range(), now);
                    }

                    let effective = self
                        .config
                        .find_did(did)
                        .map(|d| requested_interval.max(d.min_periodic_interval))
                        .unwrap_or(requested_interval);

                    self.periodic.register(did, effective, src.clone(), now);
                }
                self.pos(src, 0x2A, &[mode], now)
            }
            _ => self.nrc(src, 0x2A, H::Error::sub_function_not_supported(), now),
        }
    }

    fn on_routine_control(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        // Sub-function value is the routine control type (0x01/0x02/0x03).
        // Payload after SID: [sub_function, routine_id_high, routine_id_low, option_record...]
        let payload = frame.payload();
        if payload.len() < 3 {
            return self.nrc(
                src,
                0x31,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let sub_function = frame.sub_function_value().unwrap_or(0);
        let routine_id = u16::from_be_bytes([payload[1], payload[2]]);
        let option_record = payload.get(3..).unwrap_or(&[]);

        let mut ctx = self.create_request_context();

        let mut buf = [0u8; MAX_FRAME];
        let result = self
            .handler
            .routine_control(&mut ctx, routine_id, sub_function, option_record, &mut buf);

        self.handle_request_context(ctx);

        let len = match result {
            Ok(v) => v,
            Err(err) => {
                return self.nrc_raw(src, 0x31, err.into(), now);
            }
        };

        if suppressed {
            return Ok(());
        }

        let mut resp: Vec<u8, MAX_FRAME> = Vec::new();

        #[cfg(feature = "defmt")]
        {
            defmt::unwrap!(resp.push(sub_function));
            defmt::unwrap!(resp.push(payload[1]));
            defmt::unwrap!(resp.push(payload[2]));
            defmt::unwrap!(resp.extend_from_slice(&buf[..len]));
        }

        #[cfg(not(feature = "defmt"))]
        {
            let _ = resp.push(sub_function);
            let _ = resp.push(payload[1]);
            let _ = resp.push(payload[2]);
            let _ = resp.extend_from_slice(&buf[..len]);
        }

        self.pos(src, 0x31, &resp, now)
    }

    fn on_communication_control(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let payload = frame.payload();
        if payload.len() < 2 {
            return self.nrc(
                src,
                0x28,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let control_type = frame.sub_function_value().unwrap_or(0);
        let comm_type = payload[1];

        let mut ctx = self.create_request_context();

        let result = self.handler
            .communication_control(&mut ctx, control_type, comm_type);

        self.handle_request_context(ctx);

        if let Err(err) = result {
            return self.nrc_raw(src, 0x28, err.into(), now);
        }

        if suppressed {
            return Ok(());
        }

        self.pos(src, 0x28, &[control_type], now)
    }

    fn on_io_control(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let payload = frame.payload();
        if payload.len() < 3 {
            return self.nrc(
                src,
                0x2F,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let did = u16::from_be_bytes([payload[0], payload[1]]);
        let control_param = payload[2];
        let control_state = payload.get(3..).unwrap_or(&[]);

        let mut ctx = self.create_request_context();

        let mut buf = [0u8; MAX_FRAME];
        let result = self
            .handler
            .io_control(&mut ctx, did, control_param, control_state, &mut buf);

        self.handle_request_context(ctx);

        let len = match result {
            Ok(v) => v,
            Err(err) => {
                return self.nrc_raw(src, 0x2F, err.into(), now);
            }
        };

        let mut resp: Vec<u8, MAX_FRAME> = Vec::new();

        #[cfg(feature = "defmt")]
        {
            defmt::unwrap!(resp.push(payload[0]));
            defmt::unwrap!(resp.push(payload[1]));
            defmt::unwrap!(resp.extend_from_slice(&buf[..len]));
        }

        #[cfg(not(feature = "defmt"))]
        {
            let _ = resp.push(payload[0]);
            let _ = resp.push(payload[1]);
            let _ = resp.extend_from_slice(&buf[..len]);
        }

        self.pos(src, 0x2F, &resp, now)
    }

    fn on_request_download(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        // Payload: [data_format_identifier, address_and_length_format,
        //           memory_address..., memory_size...]
        let payload = frame.payload();
        if payload.len() < 3 {
            return self.nrc(
                src,
                0x34,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let data_format = payload[0];
        let addr_and_len_format = payload[1];
        let addr_len = (addr_and_len_format >> 4) as usize;
        let size_len = (addr_and_len_format & 0x0F) as usize;

        if payload.len() < 2 + addr_len + size_len {
            return self.nrc(
                src,
                0x34,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let memory_address = &payload[2..2 + addr_len];
        let memory_size = &payload[2 + addr_len..2 + addr_len + size_len];

        let mut ctx = self.create_request_context();

        let mut buf = [0u8; 64];
        let result = self
            .handler
            .request_download(&mut ctx, memory_address, memory_size, data_format, 0, &mut buf);

        self.handle_request_context(ctx);

        let len = match result {
            Ok(v) => v,
            Err(err) => {
                return self.nrc_raw(src, 0x34, err.into(), now);
            }
        };

        self.pos(src, 0x34, &buf[..len], now)
    }

    fn on_transfer_data(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let payload = frame.payload();
        if payload.is_empty() {
            return self.nrc(
                src,
                0x36,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let block_seq = payload[0];
        let data = payload.get(1..).unwrap_or(&[]);

        let mut ctx = self.create_request_context();

        let mut buf = [0u8; MAX_FRAME];
        let result = self
            .handler
            .transfer_data(&mut ctx, block_seq, data, &mut buf);

        self.handle_request_context(ctx);

        let len = match result {
            Ok(v) => v,
            Err(err) => {
                return self.nrc_raw(src, 0x36, err.into(), now);
            }
        };

        let mut resp: Vec<u8, MAX_FRAME> = Vec::new();

        #[cfg(feature = "defmt")]
        {
            defmt::unwrap!(resp.push(block_seq));
            defmt::unwrap!(resp.extend_from_slice(&buf[..len]));
        }

        #[cfg(not(feature = "defmt"))]
        {
            let _ = resp.push(block_seq);
            let _ = resp.extend_from_slice(&buf[..len]);
        }

        self.pos(src, 0x36, &resp, now)
    }

    fn on_transfer_exit(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let parameter_record = frame.payload();

        let mut ctx = self.create_request_context();

        let mut buf = [0u8; MAX_FRAME];
        let result = self
            .handler
            .request_transfer_exit(&mut ctx, parameter_record, &mut buf);

        self.handle_request_context(ctx);

        let len = match result {
            Ok(v) => v,
            Err(err) => {
                return self.nrc_raw(src, 0x37, err.into(), now);
            }
        };

        self.pos(src, 0x37, &buf[..len], now)
    }

    fn on_file_transfer(
        &mut self,
        src: &NodeAddress,
        frame: &UdsFrame<'_>,
        _suppressed: bool,
        now: Instant,
    ) -> Result<(), ServerError<H::Error>> {
        let payload = frame.payload();
        if payload.len() < 3 {
            return self.nrc(
                src,
                0x38,
                H::Error::incorrect_message_length_or_invalid_format(),
                now,
            );
        }

        let operation = payload[0];
        // Bytes 1-2 are file path length (big-endian u16), rest is path
        let path_len = u16::from_be_bytes([payload[1], payload[2]]) as usize;
        let path = payload.get(3..3 + path_len).unwrap_or(&[]);

        let mut ctx = self.create_request_context();

        let mut buf = [0u8; MAX_FRAME];
        let result = self
            .handler
            .request_file_transfer(&mut ctx, operation, path, &mut buf);

        self.handle_request_context(ctx);

        let len = match result {
            Ok(v) => v,
            Err(err) => {
                return self.nrc_raw(src, 0x38, err.into(), now);
            }
        };

        self.pos(src, 0x38, &buf[..len], now)
    }

    // endregion: Service handlers

    // region: Periodic dispatch

    fn dispatch_periodic(&mut self, now: Instant) -> Result<(), ServerError<H::Error>> {
        let mut due: Vec<(u16, NodeAddress), MAX_PERIODIC> = Vec::new();
        self.periodic.collect_due(now, &mut due);

        for (did, client) in &due {
            let mut ctx = self.create_request_context();

            let mut data_buf = [0u8; MAX_FRAME];
            let result = self
                .handler
                .read_did(&mut ctx, *did, &mut data_buf);

            self.handle_request_context(ctx);

            let len = match result {
                Ok(v) => v,
                Err(err) => {
                    let nrc = err.into();

                    #[cfg(feature = "defmt")]
                    defmt::error!("periodic error: {=u8}", nrc);

                    self.periodic.cancel(*did, client);
                    continue;
                }
            };

            // [periodic_data_identifier (1 byte), data_record (n bytes)]
            let did_low = (*did & 0xFF) as u8;
            let mut frame = Vec::new();

            #[cfg(feature = "defmt")]
            {
                defmt::unwrap!(frame.push(did_low));
                defmt::unwrap!(frame.extend_from_slice(&data_buf[..len]));
            }

            #[cfg(not(feature = "defmt"))]
            {
                let _ = frame.push(did_low);
                let _ = frame.extend_from_slice(&data_buf[..len]);
            }


            self.enqueue(client.clone(), frame)?;
            self.periodic.advance(*did, client, now);
        }

        Ok(())
    }

    fn create_request_context(&mut self) -> UdsRequestContext<S> {
        UdsRequestContext {
            session: self.session.clone(),
            security_provider: self.security_provider.clone(),
            drop_requests: false,
            clear_outbox: false,
            reset_security_state: false
        }
    }

    fn handle_request_context(&mut self, ctx: UdsRequestContext<S>) {
        if ctx.session != self.session {
            self.session.session_type = ctx.session.session_type;
            self.session.security_level = ctx.session.security_level;
        }
        if ctx.clear_outbox {
            self.outbox.clear();
        }
        if ctx.reset_security_state {
            self.security.reset();
        }
        if ctx.drop_requests {
            self.drop_requests = true;
        }
    }

    // endregion: Periodic dispatch
}

// endregion: UdsServer

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UdsRequestContext<T>
where
    T: SecurityProvider
{
    pub session: SessionState,
    pub security_provider: T,
    pub(crate) drop_requests: bool,
    pub(crate) clear_outbox: bool,
    pub(crate) reset_security_state: bool,
}

impl<T> UdsRequestContext<T>
where
    T: SecurityProvider
{
    pub fn drop_requests(&mut self) {
        self.drop_requests = true;
    }

    pub fn clear_outbox(&mut self) {
        self.clear_outbox = true;
    }

    pub fn reset_security_state(&mut self) {
        self.reset_security_state = true;
    }
}