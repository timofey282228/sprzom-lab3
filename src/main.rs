use std::error::Error;
use std::str::FromStr;
use vl_big_ints::UnsignedLongInt;
use gf2::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Prameters: targeting {M}, closest storage size {DIM}; \n\ttotal bits used per element struct: {}", usize::BITS as usize *DIM);
    let a = GF2Element::from_str("0666B51F56462D1588CBDA04433290E4BECDBC15CDFDA313CC0FF13D3E7EFA9F67584A2A72A08FCB77B1DBD8049C06E77EB16DEDA6")?;
    let b = GF2Element::from_str("04C2B6619096510EBDB44DBEAD30DE6A2FDA6473EE664B40FFFE6559A57FB5BABF4D43E087C74E99B98B16C9AFD35D21C1D2413B6F")?;
    let n = UnsignedLongInt::from_str("0314838BB0E599D370485AC3DBA721D9F32F60FA2FAC8F8BB49A52199A8A7745187CA8C7AFC8377D96572379EAA40A01DF54E6DEC3")?;

    let a_add_b = a.add(&b);
    let a_mul_b = a.mul(&b);
    let a_sqr = a.sqr();
    let a_pow_n = a.pow(&n);
    let a_inv = a.inverse();
    let a_trace = a.trace();

    println!("a + b: {:x}", &a_add_b);
    println!("a * b: {:x}", &a_mul_b);
    println!("a^2: {:x}", &a_sqr);
    println!("a^n: {:x}", &a_pow_n);
    println!("Tr(a): {:x}", &a_trace);
    println!("a^-1: {:x}", &a_inv);

    Ok(())
}