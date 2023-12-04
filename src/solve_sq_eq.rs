use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::{GF2Element, DIM, MOD_DEG};

#[derive(PartialOrd, PartialEq, Eq, Debug)]
pub enum SolveEqError {
    NoSolution,
}

impl Display for SolveEqError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSolution => write!(f, "This equation has no solution")
        }
    }
}

impl Error for SolveEqError {}

pub fn halftrace(el: &GF2Element<DIM>) -> GF2Element<DIM> {
    let mut s = el.clone();
    let mut out = GF2Element::ZERO;
    for i in 0..MOD_DEG {
        if i%2 == 0 {
            out = out.add(&s);
        }
        s = s.sqr();
    }
    out
}

pub fn solve(a: &GF2Element<DIM>, b: &GF2Element<DIM>) -> Result<(GF2Element<DIM>, GF2Element<DIM>), SolveEqError> {
    let c = b.mul(&a.sqr().inverse());
    if c.trace() != GF2Element::ZERO{
        return Err(SolveEqError::NoSolution);
    }

    let z1 = halftrace(&c);
    let z2 = z1.add(&GF2Element::ONE);

    let x1 = z1.mul(&a);
    let x2 = z2.mul(&a);

    return Ok(
        (x1, x2)
    )
}

#[cfg(test)]
mod tests{
    use std::error::Error;
    use std::str::FromStr;
    use crate::GF2Element;
    use crate::solve_sq_eq::solve;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>>{
        let a = GF2Element::from_str("050E04B10B1CA453CDA09E9F9EAF055A3F6EE35A162EC9E390CC67888B2FDE0EA1DD5B2A2C9E6E373F3DE18B5621FF810F0C338D71")?;
        let b = GF2Element::from_str("066A1CDA81DFBD5953500236E1D5264911779ECCBCBF1241AC2886FF71AB374B7DD0A28E6863801FF40507229FE65223587491D2CD")?;

        let (x1, x2) = solve(&a, &b)?;
        println!("{:x}", &x1);
        println!("{:x}", &x2);
        assert_eq!(GF2Element::ZERO, x1.sqr().add(&a.mul(&x1)).add(&b));
        assert_eq!(GF2Element::ZERO, x2.sqr().add(&a.mul(&x2)).add(&b));
        Ok(())
    }
}