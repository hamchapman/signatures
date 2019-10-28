//! secp256k1 elliptic curve

use super::Curve;
use generic_array::typenum::U32;

/// The secp256k1 elliptic curve: y² = x³ + 7 over a ~256-bit prime field.
/// Specified in Certicom's SECG in SEC 2: Recommended Elliptic Curve Domain Parameters:
///
/// <http://www.secg.org/sec2-v2.pdf>
///
/// This curve is most notable for its use in Bitcoin and other cryptocurrencies.
#[derive(Debug, Default)]
pub struct Secp256k1;

impl Curve for Secp256k1 {
    /// 256-bit (32-byte) private scalar
    type ScalarSize = U32;
}

/// Fixed-sized (a.k.a. "raw") secp256k1 ECDSA signature
pub type FixedSignature = crate::fixed_signature::FixedSignature<Secp256k1>;