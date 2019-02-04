/// The size of value
pub const V_SIZE: usize = 8;
/// The size of random commitment
pub const R_SIZE: usize = 32;
/// The size of plain text
pub const PLAINTEXT_SIZE: usize = V_SIZE + R_SIZE;
/// The size of cipher text
pub const CIPHERTEXT_SIZE: usize = V_SIZE + R_SIZE;
// BLAKE2s invocation personalizations
/// BLAKE2s Personalization for CRH^ivk = BLAKE2s(ak | nk)
pub const KDF_PERSONALIZATION: &'static [u8; 8] = b"zech_KDF";     
pub const KEY_DIVERSIFICATION_PERSONALIZATION: &'static [u8; 8] = b"zech_div";
pub const CRH_IVK_PERSONALIZATION: &'static [u8; 8] = b"zech_ivk";
pub const MIMC_PERSONALIZATION: &'static [u8; 8] = b"zechMIMC";
pub const DEFAULT_MIMC_SEED: &[u8] = b"mimc";
pub const DEFAULT_MIMC_ROUND: usize = 97;
pub const DEFAULT_MIMC_EXPONENT: u64 = 7;