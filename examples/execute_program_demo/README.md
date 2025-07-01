# execute_program Demo

This example demonstrates how to use the `execute_program` function from the Noir tooling to execute ACIR programs.

## What it does

This demo creates a simple ACIR program that adds two field elements and shows how to:
1. Set up the execution environment
2. Create input witnesses
3. Execute the program using `execute_program`
4. Extract and display results

## How to run

From the Noir repository root:

```bash
cd examples/execute_program_demo
cargo run
```

## Expected output

```
=== Noir execute_program Demo ===
Input values: 5 + 10
Execution successful! Result: 15
Expected: 15 (5 + 10)
Witness contains 4 values
```

## Key concepts demonstrated

- **ACIR Program**: Creating a minimal circuit representation
- **Witness Map**: Encoding inputs for circuit execution
- **Black Box Solver**: Using BN254 solver for cryptographic operations
- **Foreign Call Executor**: Handling external function calls
- **Witness Stack**: Extracting execution results

## Note

This is a simplified example for demonstration purposes. In practice, you would typically:
1. Compile a Noir program to get the ACIR
2. Parse inputs from a Prover.toml file
3. Use the program's ABI to encode inputs
4. Execute and decode outputs using the ABI

## Dependencies

- `acvm`: The ACVM (Algebraic Circuit Virtual Machine) core
- `bn254_blackbox_solver`: BN254 curve black box function solver
- `nargo`: The Noir tooling library containing `execute_program`