// env RUSTFLAGS="-L <mcl>/lib" cargo run
use mcl_rust::*;

#[allow(non_snake_case)]
fn main() {
    println!("mcl version={:04x}", get_version());
    let b = init(CurveType::SNARK);
    if !b {
        println!("init err");
    }
    let mut x = Fr::zero();
    println!("x={}", x.get_str(10));
    x.set_int(123456);
    println!("x={}", x.get_str(10));
    x.set_int(0xfff);
    println!("x={}", x.get_str(16));
    x.clear();
    println!("x={}", x.get_str(10));
    x.set_str("0x123", 0);
    println!("x={}", x.get_str(16));
    let buf = x.serialize();
    println!("serialize={:x?}", buf); // put hex byte
    let mut y = Fr::zero();
    if y.deserialize(&buf) {
        println!("y={}", y.get_str(16));
    } else {
        println!("err deserialize");
    }
    if x != y {
        println!("ng");
    }
    x.set_int(1);
    if x == y {
        println!("ng");
    }
    if !x.is_one() {
        println!("ng");
    }
    x.set_int(123);
    y.set_int(567);
    let mut z = unsafe { Fr::uninit() };
    Fr::add(&mut z, &x, &y);

    let x1 = Fr::from_str("1234", 10).unwrap();
    println!("x1={}", x1.get_str(10));

    println!("z={}", z.get_str(10));
    println!("x={}", x.get_str(10));
    println!("y={}", y.get_str(10));

    let mut P1 = unsafe { G1::uninit() };
    let mut P2 = unsafe { G1::uninit() };
    let mut Q1 = unsafe { G2::uninit() };
    let mut Q2 = unsafe { G2::uninit() };
    let mut e1 = unsafe { GT::uninit() };
    let mut e2 = unsafe { GT::uninit() };
    let mut e3 = unsafe { GT::uninit() };
    P1.set_hash_of("abc".as_bytes());
    Q1.set_hash_of("abc".as_bytes());
    pairing(&mut e1, &P1, &Q1);
    x.set_by_csprng();
    y.set_by_csprng();
    G1::mul(&mut P2, &P1, &x);
    G2::mul(&mut Q2, &Q1, &y);
    pairing(&mut e2, &P2, &Q2);
    GT::pow(&mut e3, &e1, &x);
    GT::pow(&mut e1, &e3, &y);
    if e1 == e2 {
        println!("ok");
    } else {
        println!("ng");
    }

    // let x = Fr::from_str("21888242871839275222246405745257275088548364400416034343698204186575808495616", 10).unwrap();
    // let y = Fr::from_str("2", 10).unwrap();
    // let mut z = unsafe { Fr::uninit() };
    // Fr::add(&mut z, &x, &y);
    // println!("{:?}",z.get_str(10));
    //
    // let random_el = make_random_elements(10);
    // for e in random_el.iter(){
    //     println!("{:?}",e.get_str(16));
    // }
    //
    // let ffr_vec = convert_elements(&random_el);
    // for e in ffr_vec.iter() {
    //     println!("{:?}",e);
    // }


}

// use pairing_ce::bn256::Fr as FFr;
// use pairing_ce::ff::{Field,PrimeField};
// fn make_random_elements(num_elements: usize) -> Vec<Fr> {
//     let mut result = vec![Fr::zero(); num_elements];
//     for i in 0..num_elements {
//         result[i].set_by_csprng();
//     }
//
//     result
// }
//
// fn convert_elements(fr_vec: &Vec<Fr>) -> Vec<FFr> {
//     let mut r: Vec<FFr> = Vec::with_capacity(fr_vec.len());
//     unsafe{r.set_len(fr_vec.len())};
//
//     for (from, mut to) in fr_vec.iter().zip(r.iter_mut()) {
//         let s = from.get_str(10);
//         *to = FFr::from_str(&s).unwrap();
//     }
//
//     r
// }
//
// #[test]
// fn test_bench_fr_vs_ffr(){
//     use std::time::Instant;
//     let b = init(CurveType::SNARK);
//     if !b {
//         println!("init err");
//     }
//
//     //generate test data
//     let num_elements = 10000000usize;
//     let x = make_random_elements(num_elements);
//     let y = make_random_elements(num_elements);
//
//     // mcl
//     let mut z = vec![Fr::zero(); num_elements];
//
//     let now = Instant::now();
//     for i in 0..num_elements{
//         Fr::mul(&mut z[i], &x[i], &y[i]);
//     }
//     let total_proving = now.elapsed();
//     let proving_avg = total_proving / num_elements as u32;
//     let proving_avg = proving_avg.as_nanos();
//     println!("mcl taken {:?} ns", proving_avg);
//
//     let mut zz = convert_elements(&x);
//     let yy = convert_elements(&y);
//     let now = Instant::now();
//     for i in 0..num_elements{
//        zz[i].mul_assign(&yy[i]);
//     }
//     let total_proving = now.elapsed();
//     let proving_avg = total_proving / num_elements as u32;
//     let proving_avg = proving_avg.as_nanos();
//     println!("mcl taken {:?} ns", proving_avg);
// }
