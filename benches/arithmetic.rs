use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;

#[derive(serde_derive::Deserialize)]
struct TestCase {
    base: String,
    exponent: String,
    modulus: String,
}

fn run_test_case(test_case: &str) {
    let test_case: TestCase = serde_json::from_str(test_case).unwrap();
    let base = BigUint::from_bytes_be(&array_bytes::hex2bytes_unchecked(test_case.base));
    let exponent = BigUint::from_bytes_be(&array_bytes::hex2bytes_unchecked(test_case.exponent));
    let modulus = BigUint::from_bytes_be(&array_bytes::hex2bytes_unchecked(test_case.modulus));

    base.modpow(&exponent, &modulus);
}

fn test_16_rsa(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "FC83",
            "exponent": "E4B3",
            "modulus":  "CFDD"
        }
    "#;

    c.bench_function("16 bits rsa", |b| b.iter(|| run_test_case(test_case)));
}

fn test_16_pow2(c: &mut Criterion) {
    let test_case = r#"
         {
            "base":     "FC83",
            "exponent": "E4B3",
            "modulus":  "1000"
         }
     "#;

    c.bench_function("16 pow two", |b| b.iter(|| run_test_case(test_case)));
}

fn test_32_rsa(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "C1157599",
            "exponent": "B582C905",
            "modulus":  "D8BEA141"
        }
    "#;

    c.bench_function("32 bits rsa", |b| b.iter(|| run_test_case(test_case)));
}

fn test_32_pow2(c: &mut Criterion) {
    let test_case = r#"
         {
            "base":     "C1157599",
            "exponent": "B582C905",
            "modulus":  "10000000"
         }
     "#;

    c.bench_function("32 pow two", |b| b.iter(|| run_test_case(test_case)));
}

fn test_64_rsa(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "DBBD676B1D8ECD45",
            "exponent": "EEF948E552EB6285",
            "modulus":  "EA68D469703323A9"
        }
    "#;

    c.bench_function("64 bits rsa", |b| b.iter(|| run_test_case(test_case)));
}

fn test_64_pow2(c: &mut Criterion) {
    let test_case = r#"
         {
            "base":     "DBBD676B1D8ECD45",
            "exponent": "EEF948E552EB6285",
            "modulus":  "1000000000000000"
         }
     "#;

    c.bench_function("64 pow two", |b| b.iter(|| run_test_case(test_case)));
}

fn test_128_rsa(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "C0C92D67D21A4600655F09C589D08449",
            "exponent": "BBC56904912C9B66489974565DE8FA7F",
            "modulus":  "ED652CD67959A620067A59A18B20325B"
        }
    "#;

    c.bench_function("128 bits rsa", |b| b.iter(|| run_test_case(test_case)));
}

fn test_128_pow2(c: &mut Criterion) {
    let test_case = r#"
         {
             "base":     "C0C92D67D21A4600655F09C589D08449",
             "exponent": "BBC56904912C9B66489974565DE8FA7F",
             "modulus":  "10000000000000000000000000000000"
         }
     "#;

    c.bench_function("128 pow two", |b| b.iter(|| run_test_case(test_case)));
}

fn test_256_rsa(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "BEC41460FFD701CAF53EAEE633F794BCB833552BB98CE2DF1879C2FFA1109773",
            "exponent": "CE712DD15B07E3A0F7819DE5F19A5DE04B90A3FA6CBFDEDDEE7F2AF8369390DF",
            "modulus":  "A4ABA11D64C81C5C92D9F051FC28F83738D79911A5A98007D22C927D366C082B"
        }
    "#;

    c.bench_function("256 bits rsa", |b| b.iter(|| run_test_case(test_case)));
}

fn test_256_pow2(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "BEC41460FFD701CAF53EAEE633F794BCB833552BB98CE2DF1879C2FFA1109773",
            "exponent": "CE712DD15B07E3A0F7819DE5F19A5DE04B90A3FA6CBFDEDDEE7F2AF8369390DF",
            "modulus":  "1000000000000000000000000000000000000000000000000000000000000000"
        }
    "#;

    c.bench_function("256 pow two", |b| b.iter(|| run_test_case(test_case)));
}

fn test_512_rsa(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "BD138AD4420FBC808CDBB9A536B3988CA352EF911DBA6738CBB2AC900CD0044857310FA5CECE626DE01AF622DD08686C47E1CCBDC9E0C6E3818872A2A26F2C4F",
            "exponent": "AA5E43B462897298946C947043AE741DE2F649D7F675E474B1EE46198773348DA0921F370E36D13CB1F512519BD7D1753F9AA8393B94A5992E539B0548660E95",
            "modulus":  "A4CFAF84B4FECE9732A984F4760B5F66E871881899166A651F115AFBD3DDF39083FB1C5116AA8264CDCCCC18E5E2A119731D995061CDE6CC7A07BC9FA958C8D3"
        }
    "#;

    c.bench_function("512 bits rsa", |b| b.iter(|| run_test_case(test_case)));
}

fn test_512_pow2(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "BD138AD4420FBC808CDBB9A536B3988CA352EF911DBA6738CBB2AC900CD0044857310FA5CECE626DE01AF622DD08686C47E1CCBDC9E0C6E3818872A2A26F2C4F",
            "exponent": "AA5E43B462897298946C947043AE741DE2F649D7F675E474B1EE46198773348DA0921F370E36D13CB1F512519BD7D1753F9AA8393B94A5992E539B0548660E95",
            "modulus":  "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        }
    "#;

    c.bench_function("512 pow two", |b| b.iter(|| run_test_case(test_case)));
}

fn test_1024_rsa(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "D45CA27B609309370EF4F49F0E3D9C99B68FDBC562A0DDA1EEA101694C4F0C2461534C41FBF1720500924D710195A8A8B9450681BDE428B580ADF6BBB1078D168412F4BB567B7ACB4303CE9073CBAB39A75F5B44469FC06AA3FE8AEA26C439DB9F3572E5A91CF6E823B1289DBA1D2BA44121147B9B5588C230F55DD0CE416DFD",
            "exponent": "B0949D80408138858F577ED9BBE1E8F18B254627C1A6C492D12F5A30FAC9646747262E2117D38C29FD545ED236E9A37CF7224153B2A2845B14B5CDF55BDC882A69E9FA2F44D08FA53228F935873B76063F503806DE249BA4282621F21D10E3C0A6EA3983985D6BF3A7D6C409986DFC011D2938BCE9B3A386FCD7911F6AA7AD3D",
            "modulus":  "ADC11E07F5ED75E018C1ACDF3A10C96D3B7E2E316184A648EBD170240BDA52CE59F899C19AE95C52ACFE0E15E530394C687C3ABC0FE3CF7AA247DAA6583BA90FF21DE91D44DCAA5FAA8C09F35FC12749075EB64FE07F538EFB792DBA290F592AB3483E9CAF980CFC11AEFAEB99779BC7E0231BF1BBA7B2806B5D764613212669"
        }
    "#;

    c.bench_function("1024 bits rsa", |b| b.iter(|| run_test_case(test_case)));
}

fn test_1024_pow2(c: &mut Criterion) {
    let test_case = r#"
        {
            "base":     "D45CA27B609309370EF4F49F0E3D9C99B68FDBC562A0DDA1EEA101694C4F0C2461534C41FBF1720500924D710195A8A8B9450681BDE428B580ADF6BBB1078D168412F4BB567B7ACB4303CE9073CBAB39A75F5B44469FC06AA3FE8AEA26C439DB9F3572E5A91CF6E823B1289DBA1D2BA44121147B9B5588C230F55DD0CE416DFD",
            "exponent": "B0949D80408138858F577ED9BBE1E8F18B254627C1A6C492D12F5A30FAC9646747262E2117D38C29FD545ED236E9A37CF7224153B2A2845B14B5CDF55BDC882A69E9FA2F44D08FA53228F935873B76063F503806DE249BA4282621F21D10E3C0A6EA3983985D6BF3A7D6C409986DFC011D2938BCE9B3A386FCD7911F6AA7AD3D",
            "modulus":  "1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        }
    "#;

    c.bench_function("1024 pow two", |b| b.iter(|| run_test_case(test_case)));
}

criterion_group!(
    benches,
    test_16_rsa,
    test_16_pow2,
    test_32_rsa,
    test_32_pow2,
    test_64_rsa,
    test_64_pow2,
    test_128_rsa,
    test_128_pow2,
    test_256_rsa,
    test_256_pow2,
    test_512_rsa,
    test_512_pow2,
    test_1024_rsa,
    test_1024_pow2,
);
criterion_main!(benches);
