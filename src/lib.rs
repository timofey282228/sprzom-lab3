use std::str::FromStr;
use vl_big_ints::UnsignedLongInt;

// Task specific
pub const M: usize = 419;

pub const DIM: usize = (M + usize::BITS as usize - 1) / (usize::BITS as usize);
const MOD_DIM: usize = 7;
pub const MOD: [usize; MOD_DIM] = [0x204003, 0, 0, 0, 0, 0, 0x800000000];
pub const MOD_DEG: usize = 419;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GF2Element<const DIM: usize> {
    // TODO make private
    pub data: [usize; DIM],
}

impl GF2Element<DIM> {
    pub const ZERO: Self = GF2Element {
        data: [0; DIM]
    };

    pub const ONE: Self = GF2Element {
        data: [0x1, 0, 0, 0, 0, 0, 0]
    };
    pub fn new() -> Self {
        Self {
            data: [0; DIM]
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut newpol = Self::new();

        for i in 0..DIM {
            newpol.data[i] = self.data[i] ^ other.data[i];
        }

        newpol
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut temp_array = [0; DIM * 2];

        for i in 0..DIM * usize::BITS as usize {
            for j in 0..DIM * usize::BITS as usize {
                temp_array[(i + j) / usize::BITS as usize] ^=
                    (
                        ((self.data[i / usize::BITS as usize] & (1 << (i % usize::BITS as usize))) >> (i % usize::BITS as usize)) &
                            ((other.data[j / usize::BITS as usize] & (1 << (j % usize::BITS as usize))) >> (j % usize::BITS as usize))
                    ) << 1 * ((i + j) % usize::BITS as usize);
            }
        }

        let data = modulo(temp_array);

        Self { data }
    }

    #[inline]
    pub fn get_coef(&self, n: usize) -> bool {
        if self.data[n / DIM] & (1 << n % usize::BITS as usize) != 0 { true } else { false }
    }

    pub fn deg(&self) -> usize {
        deg(&self.data)
    }

    // TODO improve performance?
    pub fn pow(&self, e: &UnsignedLongInt) -> Self {
        if e == &UnsignedLongInt::from(0) {
            return Self::ONE;
        }

        let mut result = GF2Element::ONE;
        for i in (0..e.get_highest_set_bit().expect("must not be 0 at this point") + 1).rev() {
            if e.get_bit(i) {
                result = result.mul(&self);
            }
            if i != 0 {
                result = result.sqr();
            }
        }

        result
    }


    // TODO improve performance
    pub fn sqr(&self) -> Self {
        self.mul(self)
    }

    // TODO improve performance
    pub fn trace(&self) -> Self {
        // Horner's scheme-like
        let mut out = self.sqr();
        for i in 2..MOD_DEG {
            out = out.add(self).sqr();
        }

        out.add(self)
    }

    // TODO improve performance
    pub fn inverse(&self) -> Self {
        let mut out = self.sqr();
        for i in 2..MOD_DEG {
            out = out.mul(self).sqr();
        }

        out
    }
}

impl From<[usize; 2*DIM]> for GF2Element<DIM>{
    fn from(value: [usize; 2*DIM]) -> Self {
        Self{data: modulo(value)}
    }
}

fn modulo(x: [usize; 2 * DIM]) -> [usize; DIM] {
    let mut x: [usize; 2 * DIM] = x;
    if deg(&x) < MOD_DEG {
        let mut result = [0; DIM];
        for i in 0..DIM {
            result[i] = x[i];
        }
        return result;
    }

    loop {
        let x_deg = deg(&x);
        if x_deg < MOD_DEG {
            break;
        }

        // shift bits
        let to_shift_total = x_deg - MOD_DEG;
        let to_shift_bits = to_shift_total % usize::BITS as usize;
        let to_shift_digits = to_shift_total / usize::BITS as usize;

        let mut local_mod_copy = [0; 2 * DIM];

        for i in to_shift_digits..MOD_DIM + to_shift_digits {
            local_mod_copy[i] = MOD[i - to_shift_digits];
        }

        if to_shift_bits > 0 {
            let mut prev = 0usize;
            for i in 0..2 * DIM {
                // select usize::BITS - to_shift_bits and save them
                let save = (local_mod_copy[i] & (((1 << to_shift_bits) - 1) << (usize::BITS as usize - to_shift_bits))) >> (usize::BITS as usize - to_shift_bits);
                local_mod_copy[i] <<= to_shift_bits;
                local_mod_copy[i] |= prev;
                prev = save;
            }
        }

        for i in 0..2 * DIM {
            x[i] = x[i] ^ local_mod_copy[i];
        }
    }

    let mut result = [0; DIM];
    for i in 0..DIM {
        result[i] = x[i];
    }

    result
}

fn deg(arr: &[usize]) -> usize {
    let mut maxpart = 0;
    for i in (0..arr.len()).rev() {
        if arr[i] > 0 {
            maxpart = i;
            break;
        }
    }

    if arr[maxpart] == 0 {
        return 0;
    }

    let mut biti = usize::BITS - 1;
    while biti >= 0 {
        if arr[maxpart] & (1 << biti) != 0 {
            return maxpart * usize::BITS as usize + biti as usize;
        }
        biti -= 1
    }

    panic!("implementation error")
}

mod display;
mod from_str;
mod solve_sq_eq;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::*;

    #[test]
    fn deg_test() -> Result<(), Box<dyn Error>> {
        println!("{}", deg(&MOD));
        assert_eq!(deg(&MOD), 419);
        Ok(())
    }

    #[test]
    fn mul_test() -> Result<(), Box<dyn Error>> {
        let a = GF2Element::from_str("009F3DB3DA40977DC47CA6385DE4F71AC696344C5F4A476619D4EF4D4903AF3EC84B7E40C3E41E14F1AF9AEBE60831234A42B286FE")?;
        println!("{:x}", &a);
        println!("{:b}", &a);
        for field in &a.data {
            println!("{:64b}", &field)
        }
        let b = GF2Element::from_str("0644495AC876BCB7DE7B27DA630E1B4D9E6EFA674961B12F23E372C3D0E3A8C6F635AA501685A06117EAD666DD6F15C9C79BB1E831")?;
        let c = a.mul(&b);
        println!("{:x}", &c);

        assert_eq!(c, GF2Element::from_str("03DFE436346D171186791672E60C881CAA0CEE01B0320A8DE7888C8749E95D32D153A872FB2F43E943B30F54460E49310B0631858D")?);

        Ok(())
    }

    #[test]
    fn modulo_test() -> Result<(), Box<dyn Error>> {
        let p = GF2Element::from_str("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")?;
        let r = GF2Element::from_str("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc183fde")?;
        assert_eq!(p, r);

        Ok(())
    }

    #[test]
    fn sqr_test() -> Result<(), Box<dyn Error>> {
        let p = GF2Element::from_str("009F3DB3DA40977DC47CA6385DE4F71AC696344C5F4A476619D4EF4D4903AF3EC84B7E40C3E41E14F1AF9AEBE60831234A42B286FE")?;
        let s = p.sqr();

        assert_eq!(s, GF2Element::from_str("03B5416F82167EDAAD8508B5C83D7BBEB9858EFCA89027DDCBA86AE7FDC6FD791B784A1430C380EAEC82C86EB3528C8BB1F8CA6BF7")?);

        Ok(())
    }

    #[test]
    fn pow_test() -> Result<(), Box<dyn Error>> {
        let p = GF2Element::from_str("009F3DB3DA40977DC47CA6385DE4F71AC696344C5F4A476619D4EF4D4903AF3EC84B7E40C3E41E14F1AF9AEBE60831234A42B286FE")?;
        let e = UnsignedLongInt::from_str("044B060AED749F4637B2151CB79A40D8CF329AFCB99B19054104E5098AC91FF5C34512510BAEAEC154C1C5ABD4571873F802C5A4BF")?;

        assert_eq!(p.pow(&e), GF2Element::from_str("069B317CE6B0E05391EEF785F32BD1CEE9C172B062422CF0EE9F65153B30691F4EC762C13F5AF6D4BF77EAD7B0102736EDC520DA98")?);
        Ok(())
    }

    #[test]
    fn trace_test() -> Result<(), Box<dyn Error>> {
        let p = GF2Element::from_str("073A5F1662A5634B30ABEF467039D38245C795D6C50B1600C2DA169BB74C819156CE000BDE8DDA14C395923ABBBE4EFA8BEC80EAE9")?;
        assert_eq!(p.trace(), GF2Element::ONE);

        Ok(())
    }
    #[test]
    fn inverse_test() -> Result<(), Box<dyn Error>>{
        let p = GF2Element::from_str("073A5F1662A5634B30ABEF467039D38245C795D6C50B1600C2DA169BB74C819156CE000BDE8DDA14C395923ABBBE4EFA8BEC80EAE9")?;
        assert_eq!(p.inverse(), GF2Element::from_str("01A3B2DC0EC7F3B782D9CFB76CF4A1384813DB24DB59D9C740BF3A20AA109F216B928FE17060508F1E2E0615061DB84CBC34B82E89")?);

        Ok(())
    }

    #[test]
    fn eq1() -> Result<(), Box<dyn Error>> {
        let a = GF2Element::from_str("050E04B10B1CA453CDA09E9F9EAF055A3F6EE35A162EC9E390CC67888B2FDE0EA1DD5B2A2C9E6E373F3DE18B5621FF810F0C338D71")?;
        let b = GF2Element::from_str("066A1CDA81DFBD5953500236E1D5264911779ECCBCBF1241AC2886FF71AB374B7DD0A28E6863801FF40507229FE65223587491D2CD")?;
        let c = GF2Element::from_str("04C1190B05D7B06470D4D030368B91BF48FBC8D207BF309F7CB87C21451DABCD293D5A560A437808BDF4184C96951A1B3F698FBD70")?;

        assert_eq!(a.add(&b).mul(&c), b.mul(&c).add(&a.mul(&c)));
        Ok(())
    }

    #[test]
    fn eq2() -> Result<(), Box<dyn Error>> {
        let d = GF2Element::from_str("04C2B6619096510EBDB44DBEAD30DE6A2FDA6473EE664B40FFFE6559A57FB5BABF4D43E087C74E99B98B16C9AFD35D21C1D2413B6F")?;
        assert_eq!(GF2Element::ONE, d.pow(&(UnsignedLongInt::from(2).pow(&UnsignedLongInt::from(M as u64)).sub(&UnsignedLongInt::from(1)))));
        Ok(())
    }
}