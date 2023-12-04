use std::fmt::{Binary, Display, Formatter, LowerHex};
use crate::{DIM, GF2Element};

impl Display for GF2Element<DIM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let binstr = format!("{:064b}", self);
            let mut binstr_iter = binstr.chars().peekable();
            write!(f, "(")?;
            while let Some(b) = binstr_iter.next() {
                write!(f, "{b}")?;
                if binstr_iter.peek().is_some() {
                    write!(f, ",")?;
                }
            }
            write!(f, ")")
        } else {
            write!(f, "[")?;
            let mut pow: usize = DIM * usize::BITS as usize;
            for i in (1..DIM).rev() {
                let mut b: usize = 1 << (usize::BITS - 1);
                for _ in (0..usize::BITS).rev() {
                    pow -= 1;
                    if self.data[i] & b != 0 {
                        write!(f, "x^{pow} + ")?;
                    }
                    b >>= 1;
                }
            }

            let mut b: usize = 1 << usize::BITS - 1;
            for _ in (1..usize::BITS).rev() {
                pow -= 1;
                if self.data[0] & b != 0 {
                    write!(f, "x^{pow} + ")?;
                }
                b >>= 1;
            }

            if self.data[0] & 1 == 1 {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
            write!(f, "]")
        }
    }
}

impl Binary for GF2Element<DIM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in (0..DIM).rev() {
            if &self.data[i] != &0 {
                write!(f, "{:064b}", &self.data[i])?;
            }
        }

        Ok(())
    }
}

impl LowerHex for GF2Element<DIM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in (0..DIM).rev() {
            write!(f, "{:016x}", &self.data[i])?;
        }

        Ok(())
    }
}

pub fn dispaly_as_poly(arr: &[usize]) -> Result<String, ()>{
    let mut f = String::new();
    f.push_str("[");
    let mut pow: usize = arr.len() * usize::BITS as usize;
    for i in (1..arr.len()).rev() {
        let mut b: usize = 1 << (usize::BITS - 1);
        for _ in (0..usize::BITS).rev() {
            pow -= 1;
            if arr[i] & b != 0 {
                f.push_str(format!("x^{pow} + ").as_str());
            }
            b >>= 1;
        }
    }

    let mut b: usize = 1 << usize::BITS - 1;
    for _ in (1..usize::BITS).rev() {
        pow -= 1;
        if arr[0] & b != 0 {
            f.push_str(format!("x^{pow} + ").as_str());
        }
        b >>= 1;
    }

    if arr[0] & 1 == 1 {
        f.push_str("1");
    } else {
        f.push_str("0");
    }
    f.push_str("]");
    Ok(f)
}

#[cfg(test)]
mod tests{
    use crate::GF2Element;

    #[test]
    fn display_test() {
        let mut data = [0; 7];
        data[0] = 0b1000001;
        let p = GF2Element {
            data
        };

        println!("{}", p);
    }
}