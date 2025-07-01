# M31 Field Element Example

This example demonstrates how to work with M31 field elements in Noir.

## What is M31?

M31 refers to the finite field with modulus 2^31 - 1 = 2,147,483,647. This is a Mersenne prime field, which has some nice properties for certain cryptographic applications.

## Key Differences from BN254

- **Modulus**: M31 has a much smaller modulus (2^31 - 1) compared to BN254 (2^254 - 2^254 - 1)
- **Bit Size**: M31 uses 31 bits, while BN254 uses 254 bits
- **Operations**: M31 blackbox solver only supports basic field arithmetic, not advanced operations like:
  - Multi-scalar multiplication
  - Elliptic curve operations
  - Poseidon2 permutation

## Running the Example

```bash
cd examples/m31_field_example
cargo run
```

## What the Example Shows

1. **Basic Field Elements**: Creating zero, one, and other field elements
2. **Arithmetic Operations**: Addition, subtraction, multiplication, division
3. **Field Properties**: Modulus, maximum bits, maximum bytes
4. **Modular Arithmetic**: Demonstrating how arithmetic wraps around the modulus
5. **Limitations**: Showing what operations are not supported

## Field Element Type

```rust
type M31FieldElement = GenericFieldElement<ark_m31::Fr>;
```

This creates a field element type that operates over the M31 field (modulus 2^31 - 1).

## Use Cases

M31 fields are useful for:
- Applications requiring smaller field sizes
- Testing and development with simpler arithmetic
- Cases where the full power of BN254 is not needed
- Educational purposes to understand field arithmetic

## Limitations

The M31 blackbox solver is currently limited and does not support the advanced cryptographic operations available in the BN254 solver. For production applications requiring elliptic curve operations or advanced cryptographic primitives, use the BN254 field instead.