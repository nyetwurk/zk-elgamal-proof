//! Crate-local [`OsRng`] for `rand` 0.10 (`SysRng` + [`UnwrapErr`]).

use core::convert::Infallible;

use rand::{
    distr::{Distribution, StandardUniform},
    rand_core::{TryCryptoRng, TryRng, UnwrapErr},
    rngs::SysRng,
    RngExt,
};

/// Stateless OS-backed CSPRNG (replaces removed `rand::rngs::OsRng`).
#[derive(Clone, Copy, Debug, Default)]
pub struct OsRng;

impl TryRng for OsRng {
    type Error = Infallible;

    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        UnwrapErr(SysRng).try_next_u32()
    }

    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        UnwrapErr(SysRng).try_next_u64()
    }

    #[inline]
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        UnwrapErr(SysRng).try_fill_bytes(dst)
    }
}

impl TryCryptoRng for OsRng {}

impl OsRng {
    /// Compatibility with historical `Rng::gen` one-shot calls (`OsRng.gen::<[u8; N]>()`).
    #[inline]
    pub fn gen<T>() -> T
    where
        StandardUniform: Distribution<T>,
    {
        let mut o = OsRng;
        o.random()
    }
}
