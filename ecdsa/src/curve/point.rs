//! Compressed and uncompressed Weierstrass elliptic curve points serialized
//! according to the `Elliptic-Curve-Point-to-Octet-String` algorithm described
//! in SEC 1: Elliptic Curve Cryptography (Version 2.0) section 2.3.3 (page 10)
//!
//! <https://www.secg.org/sec1-v2.pdf>

use super::Curve;
use core::ops::Add;
use generic_array::{typenum::U1, ArrayLength, GenericArray};

/// Size of a compressed elliptic curve point for the given curve when
/// serialized using `Elliptic-Curve-Point-to-Octet-String` encoding.
pub type CompressedPointSize<ScalarSize> = <ScalarSize as Add<U1>>::Output;

/// Size of an uncompressed elliptic curve point for the given curve when
/// serialized using the `Elliptic-Curve-Point-to-Octet-String` encoding
/// (including the `0x04` tag).
pub type UncompressedPointSize<ScalarSize> = <<ScalarSize as Add>::Output as Add<U1>>::Output;

/// Compressed elliptic curve points serialized according to the
/// `Elliptic-Curve-Point-to-Octet-String` algorithm defined
/// in section 2.3.3 of SEC 1: Elliptic Curve Cryptography (Version 2.0):
///
/// <https://www.secg.org/sec1-v2.pdf>
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct CompressedCurvePoint<C: Curve>
where
    CompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    /// Raw serialized bytes of the compressed point
    bytes: GenericArray<u8, CompressedPointSize<C::ScalarSize>>,
}

impl<C: Curve> CompressedCurvePoint<C>
where
    CompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    /// Create a new compressed elliptic curve point
    pub fn from_bytes<B>(into_bytes: B) -> Option<Self>
    where
        B: Into<GenericArray<u8, CompressedPointSize<C::ScalarSize>>>,
    {
        let bytes = into_bytes.into();
        let tag_byte = bytes.as_ref()[0];

        if tag_byte == 0x02 || tag_byte == 0x03 {
            Some(Self { bytes })
        } else {
            None
        }
    }

    /// Obtain public key as a byte array reference
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert public key into owned byte array
    #[inline]
    pub fn into_bytes(self) -> GenericArray<u8, CompressedPointSize<C::ScalarSize>> {
        self.bytes
    }
}

impl<C: Curve> AsRef<[u8]> for CompressedCurvePoint<C>
where
    CompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<C: Curve> Copy for CompressedCurvePoint<C>
where
    CompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
    <CompressedPointSize<C::ScalarSize> as ArrayLength<u8>>::ArrayType: Copy,
{
}

impl<C: Curve> Clone for CompressedCurvePoint<C>
where
    CompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    fn clone(&self) -> Self {
        Self::from_bytes(self.bytes.clone()).unwrap()
    }
}

/// Uncompressed elliptic curve points serialized according to the
/// `Elliptic-Curve-Point-to-Octet-String` algorithm, including the `0x04`
/// tag identifying the bytestring as a curve point, as defined
/// in section 2.3.3 of SEC 1: Elliptic Curve Cryptography (Version 2.0):
///
/// <https://www.secg.org/sec1-v2.pdf>
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct UncompressedCurvePoint<C: Curve>
where
    <C::ScalarSize as Add>::Output: Add<U1>,
    UncompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    /// Raw serialized bytes of the uncompressed point
    bytes: GenericArray<u8, UncompressedPointSize<C::ScalarSize>>,
}

impl<C: Curve> UncompressedCurvePoint<C>
where
    <C::ScalarSize as Add>::Output: Add<U1>,
    UncompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    /// Create a new uncompressed elliptic curve point
    pub fn from_bytes<B>(into_bytes: B) -> Option<Self>
    where
        B: Into<GenericArray<u8, UncompressedPointSize<C::ScalarSize>>>,
    {
        let bytes = into_bytes.into();

        if bytes.get(0) == Some(&0x04) {
            Some(Self { bytes })
        } else {
            None
        }
    }

    /// Obtain public key as a byte array reference
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert public key into owned byte array
    #[inline]
    pub fn into_bytes(self) -> GenericArray<u8, UncompressedPointSize<C::ScalarSize>> {
        self.bytes
    }
}

impl<C: Curve> AsRef<[u8]> for UncompressedCurvePoint<C>
where
    <C::ScalarSize as Add>::Output: Add<U1>,
    UncompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl<C: Curve> Copy for UncompressedCurvePoint<C>
where
    <C::ScalarSize as Add>::Output: Add<U1>,
    UncompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
    <UncompressedPointSize<C::ScalarSize> as ArrayLength<u8>>::ArrayType: Copy,
{
}

impl<C: Curve> Clone for UncompressedCurvePoint<C>
where
    <C::ScalarSize as Add>::Output: Add<U1>,
    UncompressedPointSize<C::ScalarSize>: ArrayLength<u8>,
{
    fn clone(&self) -> Self {
        Self::from_bytes(self.bytes.clone()).unwrap()
    }
}