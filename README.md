# SP1 Fibonacci Prover

## Usage

### Build the Program

Generate an ELF file in the `target/elf-compilation` directory.

```sh
cd program
cargo prove build
```

### Test the Program Without Proving

```sh
cargo run --release -- --execute
```

### Prove the Program

```sh
cargo run --release -- --prove
```

## Results

NOTEL All tests were run on an M2 Pro in CPU mode.

### Performance Comparison (n=20)

### Iterative Approach

```txt
n: 20
algorithm: iterative
2026-01-22T08:06:28.234739Z  INFO execute: clk = 0 pc = 0x20281c
2026-01-22T08:06:28.237152Z  INFO execute: gas: 1020123
2026-01-22T08:06:28.237198Z  INFO execute: close time.busy=3.93ms time.idle=2.04µs
Program executed successfully.
n: 20
a: 6765
b: 10946
Values are correct!
Number of cycles: 9941
```

```txt
n: 20
algorithm: iterative
2026-01-22T08:05:54.934839Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:05:54.938020Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:05:55.089939Z  INFO prove_core:generate main traces: close time.busy=142ms time.idle=1.38µs index=0
2026-01-22T08:06:02.548902Z  INFO prove_core: close time.busy=7.46s time.idle=155ms
Successfully generated proof!
2026-01-22T08:06:02.704376Z  INFO verify: close time.busy=154ms time.idle=1.63µs
Successfully verified proof!
```

### Recursive Approach

```txt
n: 20
algorithm: recursive
2026-01-22T08:04:39.533111Z  INFO execute: clk = 0 pc = 0x20281c
2026-01-22T08:04:39.535465Z  INFO execute: gas: 1020123
2026-01-22T08:04:39.535509Z  INFO execute: close time.busy=3.97ms time.idle=2.38µs
Program executed successfully.
n: 20
a: 6765
b: 10946
Values are correct!
Number of cycles: 10101
```

```txt
n: 20
algorithm: recursive
2026-01-22T08:05:21.464079Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:05:21.466715Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:05:21.610687Z  INFO prove_core:generate main traces: close time.busy=134ms time.idle=1.96µs index=0
2026-01-22T08:05:28.960087Z  INFO prove_core: close time.busy=7.35s time.idle=147ms
Successfully generated proof!
2026-01-22T08:05:29.124818Z  INFO verify: close time.busy=163ms time.idle=1.58µs
Successfully verified proof!
```

### Recursive Approach with Memoization

```txt
n: 20
algorithm: memoization
2026-01-22T08:01:47.992814Z  INFO execute: clk = 0 pc = 0x20281c
2026-01-22T08:01:47.995574Z  INFO execute: gas: 1020123
2026-01-22T08:01:47.995633Z  INFO execute: close time.busy=4.29ms time.idle=1.96µs
Program executed successfully.
n: 20
a: 6765
b: 10946
Values are correct!
Number of cycles: 13546
```

```txt
algorithm: memoization
2026-01-22T08:02:15.383281Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:02:15.386079Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:02:15.554395Z  INFO prove_core:generate main traces: close time.busy=158ms time.idle=2.83µs index=0
2026-01-22T08:02:22.929861Z  INFO prove_core: close time.busy=7.38s time.idle=172ms
Successfully generated proof!
2026-01-22T08:02:23.082239Z  INFO verify: close time.busy=151ms time.idle=1.67µs
Successfully verified proof!
```

### Matrix Approach

```txt
n: 20
algorithm: matrix
2026-01-22T08:03:26.914685Z  INFO execute: clk = 0 pc = 0x20281c
2026-01-22T08:03:26.917057Z  INFO execute: gas: 1020123
2026-01-22T08:03:26.917107Z  INFO execute: close time.busy=3.86ms time.idle=2.42µs
Program executed successfully.
n: 20
a: 6765
b: 10946
Values are correct!
Number of cycles: 9918
```

```txt
n: 20
algorithm: matrix
2026-01-22T08:03:51.906630Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:03:51.909473Z  INFO prove_core: clk = 0 pc = 0x20281c
2026-01-22T08:03:52.067041Z  INFO prove_core:generate main traces: close time.busy=148ms time.idle=1.46µs index=0
2026-01-22T08:03:59.366518Z  INFO prove_core: close time.busy=7.30s time.idle=161ms
Successfully generated proof!
2026-01-22T08:03:59.519426Z  INFO verify: close time.busy=152ms time.idle=2.33µs
Successfully verified proof!
```

### Summary

| Approach        | Gas           | Cycles    | Execute Busy | Execute Idle | Prove Busy | Prove Idle | Verify Busy | Verify Idle |
| --------------- | ------------- | --------- | ------------ | ------------ | ---------- | ---------- | ----------- | ----------- |
| **Iterative**   | **1,020,123** | 9,941     | 3.93 ms      | 2.04 µs      | 7.46 s     | 155 ms     | 154 ms      | 1.63 µs     |
| **Recursive**   | **1,020,123** | 10,101    | 3.97 ms      | 2.38 µs      | 7.35 s     | **147 ms** | 163 ms      | **1.58 µs** |
| **Memoization** | **1,020,123** | 13,546    | 4.29 ms      | **1.96 µs**  | 7.38 s     | 172 ms     | **151 ms**  | 1.67 µs     |
| **Matrix**      | **1,020,123** | **9,918** | **3.86 ms**  | 2.42 µs      | **7.30 s** | 161 ms     | 152 ms      | 2.33 µs     |

Expectedly, the optimized matrix exponentiation approach is the fastest in terms of execution and proving time, followed by the iterative approach. The recursive approach with memoization is the slowest, likely because of the overhead of the memoization, which matches the SP1 and RISC Zero best-practices, explicitly warning about this.
