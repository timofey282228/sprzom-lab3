use std::error::Error;
use std::time;
use std::time::Duration;
use csv::Writer;
use std::io;
use vl_big_ints::UnsignedLongInt;
use gf2::*;

const OUTPUT_CSV: &str = "./report/data.csv";
const EXPS: usize = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Performance calculation example");
    let mut wrt = csv::Writer::from_path(OUTPUT_CSV)?;


    for op in ["+", "*", "inverse", "sqr", "pow"] {
        let (op1, op2) = stage(&mut wrt);
        let avg_dur = measure(op, &op1, &op2, &mut wrt)?;
        println!("Average for {}: {:?}", op, &avg_dur);
    }

    Ok(())
}

fn stage<T>(writer: &mut Writer<impl io::Write>) -> (Vec<T>, Vec<T>)
    where T: From<[usize; 2 * DIM]>
{
    let mut op1: Vec<T> = Vec::with_capacity(EXPS);
    let mut op2: Vec<T> = Vec::with_capacity(EXPS);

    for _ in 0..EXPS {
        let random_bytes: Vec<usize> = (0..2 * DIM).map(|_| { rand::random::<usize>() }).collect();
        let gf2 = T::from(random_bytes.try_into().unwrap());
        op1.push(gf2);
        let random_bytes: Vec<usize> = (0..2 * DIM).map(|_| { rand::random::<usize>() }).collect();
        let gf2 = T::from(random_bytes.try_into().unwrap());
        op2.push(gf2);
    }

    (op1, op2)
}

fn measure(op: &str, op1: &Vec<GF2Element<DIM>>, op2: &Vec<GF2Element<DIM>>, wrt: &mut csv::Writer<std::fs::File>) -> Result<Duration, Box<dyn Error>> {
    println!("Running {} experiments for each operation", EXPS);

    let mut durations: Vec<Duration> = Vec::with_capacity(EXPS);
    for i in 0..EXPS {
        let duration = match op {
            "+" => { measure_add(&op1[i], &op2[i]) }
            "*" => { measure_mul(&op1[i], &op2[i]) }
            "inverse" => { measure_inverse(&op1[i]) }
            "sqr" => { measure_sqr(&op1[i]) }
            "pow" => {
                let mut op2: Vec<UnsignedLongInt> = Vec::with_capacity(EXPS);
                let random_bytes: Vec<u64> = (0..DIM).map(|_| { rand::random::<u64>() }).collect();
                op2.push(UnsignedLongInt::from(random_bytes.as_slice()));

                measure_pow(&op1[i], &op2[i])
            }
            _ => panic!("must be valid op string")
        };

        durations.push(duration);
    }

    let total_duration: time::Duration = durations.iter().sum();

    let average_duration = total_duration / durations.len() as u32;

    wrt.write_record([
        op,
        format!("{}", average_duration.as_nanos()).as_str(),
    ])?;

    Ok(average_duration)
}


fn measure_add(op1: &GF2Element<DIM>, op2: &GF2Element<DIM>) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.add(op2);
    let now = time::Instant::now();

    now.duration_since(then)
}

fn measure_mul(op1: &GF2Element<DIM>, op2: &GF2Element<DIM>) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.mul(op2);
    let now = time::Instant::now();

    now.duration_since(then)
}

fn measure_sqr(op1: &GF2Element<DIM>) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.sqr();
    let now = time::Instant::now();

    now.duration_since(then)
}

fn measure_inverse(op1: &GF2Element<DIM>) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.inverse();
    let now = time::Instant::now();

    now.duration_since(then)
}

fn measure_pow(op1: &GF2Element<DIM>, op2: &UnsignedLongInt) -> time::Duration {
    let then = time::Instant::now();
    let _ = op1.pow(op2);
    let now = time::Instant::now();

    now.duration_since(then)
}
