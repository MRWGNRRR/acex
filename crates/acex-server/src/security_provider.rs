// region: SecurityError

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SeedGenerationError;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidKeyError;

// endregion: SecurityError

// region: SecurityProvider

/// Provides seed generation and key validation for UDS SecurityAccess (0x27).
///
/// The seed/key algorithm is always application-psecific. This trait allows the server to delegate
/// without knowing the algorithm.
///
/// # Security Levels
///
/// UDS security levels use odd bytes for Request Seed (0x01, 0x03, 0x05, ...) and the
/// corresponding even byte for Send Key (0x02, 0x04, 0x06, ...). The `level` parameter is always
/// the RequestSeed byte (odd).
///
/// # Simulation
///
/// In DST the implementation should derive seeds from the injected RNG so that the full exchange
/// is reproducible across simulation runs.
pub trait SecurityProvider: Clone {
    /// Generates a seed for the given security level.
    ///
    /// Writes seed bytes into `buf` and returns the number of bytes written. On real hardware the
    /// seed must be non-deterministic (hardware RNG). In simulation derive from the seeded
    /// `acex_sim::rng::Rng`.
    fn generate_seed(&mut self, level: u8, buf: &mut [u8]) -> Result<usize, SeedGenerationError>;

    /// Validates a key against the previously generated seed.
    ///
    /// Returns `Ok(())` if the key is correct.
    fn validate_key(&mut self, level: u8, seed: &[u8], key: &[u8]) -> Result<(), InvalidKeyError>;
}

// endregion: SecurityProvider
