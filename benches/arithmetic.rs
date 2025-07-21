use criterion::{criterion_group, criterion_main, Criterion};
use dashu::{integer::fast_div::ConstDivisor, integer::UBig as DashuUBig};
use ibig::{modular::ModuloRing, UBig};
use num_bigint::BigUint;
use num_traits::Num;
use rug::Integer;
use std::hint::black_box;

/// Generates a hexadecimal string of 'f's for a given bit length.
/// This represents the number 2^N - 1, a common worst-case input.
fn gen_hex_str(bits: usize) -> String {
    "f".repeat(bits / 4)
}

fn run_all_benches(c: &mut Criterion) {
    for &bits in &[256, 512, 1024, 2048, 4096] {
        // Create a new benchmark group for each bit size
        let mut group = c.benchmark_group(format!("Modular Exponentiation {}-bit", bits));
        group.sample_size(10);

        // Generate the worst-case numbers as hex strings
        let hex_str = gen_hex_str(bits);

        // --- `rug` benchmark ---
        let rug_base = Integer::from_str_radix(&hex_str, 16).unwrap();
        let rug_exp = Integer::from_str_radix(&hex_str, 16).unwrap();
        let rug_mod = Integer::from_str_radix(&hex_str, 16).unwrap();

        group.bench_function("rug", |b| {
            b.iter(|| {
                let power = rug_base.pow_mod_ref(&rug_exp, &rug_mod).unwrap();
                let result = Integer::from(power);
                black_box(result);
            })
        });

        // --- `ibig` benchmark ---
        let ibig_base = UBig::from_str_radix(&hex_str, 16).unwrap();
        let ibig_exp = UBig::from_str_radix(&hex_str, 16).unwrap();
        let ibig_mod = UBig::from_str_radix(&hex_str, 16).unwrap();

        let ring = ModuloRing::new(&ibig_mod);
        let m_base = ring.from(&ibig_base);

        group.bench_function("ibig", |b| {
            b.iter(|| {
                let result = m_base.pow(&ibig_exp);
                black_box(result);
            })
        });

        // --- `num-bigint` benchmark ---
        let num_base = BigUint::from_str_radix(&hex_str, 16).unwrap();
        let num_exp = BigUint::from_str_radix(&hex_str, 16).unwrap();
        let num_mod = BigUint::from_str_radix(&hex_str, 16).unwrap();

        group.bench_function("num-bigint", |b| {
            b.iter(|| {
                let result = num_base.modpow(&num_exp, &num_mod);
                black_box(result);
            })
        });

        // --- `dashu` benchmark ---
        let dashu_base = DashuUBig::from_str_radix(&hex_str, 16).unwrap();
        let dashu_exp = DashuUBig::from_str_radix(&hex_str, 16).unwrap();
        let dashu_mod = DashuUBig::from_str_radix(&hex_str, 16).unwrap();

        let ring = ConstDivisor::new(dashu_mod);
        let base = ring.reduce(dashu_base);

        group.bench_function("dashu", |b| {
            b.iter(|| {
                let result = base.pow(black_box(&dashu_exp)).residue();
                black_box(result);
            })
        });

        group.finish();
    }
}

criterion_group!(benches, run_all_benches);
criterion_main!(benches);
