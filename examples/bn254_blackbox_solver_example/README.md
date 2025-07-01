# BN254 BlackBox Solver Example

This example demonstrates how to use the BN254 blackbox solver's `multi_scalar_mul` function and other elliptic curve operations.

## Overview

The example shows three main operations:

1. **Simple Scalar Multiplication**: Multiply a generator point by a scalar (2)
2. **Multi-Scalar Multiplication**: Compute `1*G1 + 2*G2` where G1 and G2 are different generator points
3. **Curve Point Addition**: Add a generator point to itself (equivalent to 2*generator)

## Running the Example

From the workspace root, run:

```bash
cargo run --example bn254_blackbox_solver_example
```

Or from this directory:

```bash
cargo run
```

## Running Tests

```bash
cargo test
```

## Key Components

### Bn254BlackBoxSolver

The main solver struct that implements the `BlackBoxFunctionSolver` trait:

```rust
let solver = Bn254BlackBoxSolver(false); // pedantic_solving = false
```

### Multi-Scalar Multiplication

The `multi_scalar_mul` function takes:
- `points`: Array of curve points in the format `[x1, y1, is_infinite1, x2, y2, is_infinite2, ...]`
- `scalars_lo`: Low 128-bit limbs of scalars
- `scalars_hi`: High 128-bit limbs of scalars

Returns: `(result_x, result_y, result_is_infinite)`

### Generator Points

The example uses `derive_generators` to create deterministic curve points:

```rust
let generators = derive_generators("test_domain".as_bytes(), 1, 0);
```

### Field Elements

The example uses BN254 field elements:

```rust
type FieldElement = GenericFieldElement<ark_bn254::Fr>;
```

## Output

The example will print:
- Input points and scalars in hexadecimal format
- Results of each operation
- Any errors that occur during computation

## Notes

- The example uses `pedantic_solving = false` to avoid strict validation
- All points are on the Grumpkin curve (embedded curve in BN254)
- Scalars are split into 128-bit limbs for compatibility with the ACIR format