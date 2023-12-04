use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use super::{GF2Element, DIM, modulo};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum GF2ElementFromStrError{
    ConversionError,
    NotAnElementError
}

impl Display for GF2ElementFromStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "couldn't interpret string")
    }
}

impl Error for GF2ElementFromStrError {}

impl From<ParseIntError> for GF2ElementFromStrError {
    fn from(value: ParseIntError) -> Self {
        Self::ConversionError
    }
}

impl FromStr for GF2Element<DIM> {
    type Err = GF2ElementFromStrError;
    // for hex strings
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > DIM * usize::BITS as usize / 4 {
            return Err(GF2ElementFromStrError::NotAnElementError);
        }

        let mut data = [0; 2*DIM];

        let extra_letters = s.len() % 16;
        let fulls = s.len() / 16;
        let mut i = 0;

        for j in (0..fulls).rev() {
            data[i] = usize::from_str_radix(
                &s[extra_letters + j * 16..extra_letters + (j + 1) * 16],
                16)?;
            i += 1;
        }

        if extra_letters > 0 {
            data[i] = usize::from_str_radix(&s[0..extra_letters], 16)?;
        }

        Ok(Self {
            data: modulo(data)
        })
    }
}

impl GF2Element<DIM> {
    pub fn from_str_bin(s: &str) -> Result<Self, GF2ElementFromStrError> {
        if s.len() > DIM * usize::BITS as usize {
            return Err(GF2ElementFromStrError::NotAnElementError);
        }

        let mut data = [0; 2*DIM];

        let extra_bits = s.len() % usize::BITS as usize;
        let fulls = s.len() / usize::BITS as usize;
        let mut i = 0;

        for j in (0..fulls).rev() {
            data[i] = usize::from_str_radix(
                &s[extra_bits + j * usize::BITS as usize..extra_bits + (j + 1) * usize::BITS as usize],
                2)?;
            i += 1;
        }

        if extra_bits > 0 {
            data[i] = usize::from_str_radix(&s[0..extra_bits], 2)?;
        }

        Ok(Self {
            data: modulo(data)
        })
    }
}