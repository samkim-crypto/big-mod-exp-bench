# Big Modular Exponentiation Benchmark

This repository contains a benchmark suite for comparing the performance of modular exponentiation with large integers across four Rust crates:

- `dashu`
- `ibig`
- `num-bigint`
- `rug`

The benchmarks are performed for various bit lengths, from 256 bits up to 4096 bits, using worst-case inputs (numbers of the form 2<sup>N</sup> - 1).

---

## ⚙️ How to Run

To run the benchmarks on your own machine, simply clone the repository and use the following Cargo command:

```bash
cargo bench
```

## 📊 Benchmark Results

The following results were obtained on an **Apple MacBook Pro with an M3 Max chip**:

| Bit Length   | `dashu`       | `rug`     | `num-bigint` | `ibig`    |
| :----------- | :------------ | :-------- | :----------- | :-------- |
| **256-bit**  | **3.57 µs**   | 6.99 µs   | 24.40 µs     | 15.63 µs  |
| **512-bit**  | **5.52 µs**   | 32.47 µs  | 71.90 µs     | 102.72 µs |
| **1024-bit** | **13.74 µs**  | 198.58 µs | 361.78 µs    | 684.83 µs |
| **2048-bit** | **44.39 µs**  | 1.42 ms   | 2.52 ms      | 4.89 ms   |
| **4096-bit** | **154.01 µs** | 9.56 ms   | 20.75 ms     | 34.70 ms  |
