use core::fmt::{Formatter, Result as FormatterResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block(pub(crate) [[u8; 4]; 4]);

impl core::fmt::LowerHex for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        writeln!(f)?;
        for y in 0..4 {
            write!(f, "[")?;
            for x in 0..4 {
                write!(f, " {:x} ", self.0[x][y])?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl core::ops::BitXor for Block {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        for row in self.0.iter_mut().zip(rhs.0.iter()) {
            for item in row.0.iter_mut().zip(row.1) {
                *item.0 ^= *item.1;
            }
        }
        Self(self.0)
    }
}

impl core::ops::BitXorAssign for Block {
    fn bitxor_assign(&mut self, rhs: Self) {
        for row in self.0.iter_mut().zip(rhs.0.iter()) {
            for item in row.0.iter_mut().zip(row.1) {
                *item.0 ^= *item.1;
            }
        }
    }
}

impl Block {
    /// Constructs a new [`Block`] based off a given
    /// array
    #[must_use]
    pub fn new(input: [u8; 16]) -> Self {
        // SAFETY: Just rearranging how the bytes are laid out in memory
        Self(unsafe { core::mem::transmute(input) })
    }
    /// Constructs a new [`Block`] based off a given slice.
    /// Should `input.len() < 16`, the last part of the [`Block`]
    /// will be padded with 0.
    /// Should `input.len() > 16`, only the first 16 bytes will
    /// be used.
    #[must_use]
    pub fn from_slice_single(input: &[u8]) -> Self {
        let mut block = [0; 16];

        for (i, block_byte) in block.iter_mut().enumerate() {
            match input.get(i) {
                Some(t) => *block_byte = *t,
                None => break,
            }
        }

        Self::new(block)
    }
    pub fn from_slice_multiple(input: &[u8]) -> impl Iterator<Item = Block> + Clone + '_ {
        input.chunks(16).map(Self::from_slice_single)
    }
    /// Constructs a new [`Block`] based off a given slice.
    /// Should `s.len() < 16`, the last part of the [`Block`]
    /// will be padded with 0.
    /// Should `s.len() > 16`, only the first 16 bytes will
    /// be used.
    #[must_use]
    pub fn from_str_single(s: &str) -> Self {
        Self::from_slice_single(s.as_bytes())
    }
    /// Returns an iterator over multiple [`Block`]s built by
    /// repeatedly calling [`Block::from_str_single`]
    pub fn from_str_multiple(s: &str) -> impl Iterator<Item = Block> + Clone + '_ {
        Self::from_slice_multiple(s.as_bytes())
    }
}

impl core::str::FromStr for Block {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_single(s))
    }
}
