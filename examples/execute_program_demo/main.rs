// use acvm::{FieldElement, BlackBoxFunctionSolver};
// use acvm::acir::circuit::{Circuit, Program};
// use acvm::acir::native_types::{WitnessMap, WitnessStack};
// use bn254_blackbox_solver::Bn254BlackBoxSolver;
use m31_blackbox_solver::M31BlackBoxSolver;
// use nargo::ops::execute_program;
// use nargo::foreign_calls::DefaultForeignCallBuilder;
// use nargo::NargoError;

/// A simple example demonstrating how to use the execute_program function
/// This creates a minimal ACIR program and executes it
fn main() {

    let result = M31BlackBoxSolver::hello_world();
    println!("M31 BlackBoxSolver says: {}", result);
    
    // println!("=== Noir execute_program Demo ===");
    
    // // Demonstrate the M31 black box solver's hello world function
    // println!("Calling M31 BlackBoxSolver hello world function:");
    // let hello_world_result = M31BlackBoxSolver::hello_world();
    // println!("M31 BlackBoxSolver says: {}", hello_world_result);
    
    // // Create a simple ACIR program that adds two field elements
    // let program = create_simple_addition_program();
    
    // // Create input witness with two values to add
    // let mut initial_witness = WitnessMap::new();
    // initial_witness.insert(1, FieldElement::from(5)); // First input
    // initial_witness.insert(2, FieldElement::from(10)); // Second input
    
    // println!("\nInput values: 5 + 10");
    
    // // Set up the execution environment with BN254 solver
    // let blackbox_solver = Bn254BlackBoxSolver::default();
    // let mut foreign_call_executor = DefaultForeignCallBuilder::default().build();
    
    // // Execute the program
    // let witness_stack = execute_program(
    //     &program,
    //     initial_witness,
    //     &blackbox_solver,
    //     &mut foreign_call_executor,
    // )?;
    
    // // Extract the result
    // let main_witness = witness_stack
    //     .peek()
    //     .expect("Should have at least one witness on the stack")
    //     .witness
    //     .clone();
    
    // // The result should be in witness index 3 (based on our simple circuit)
    // if let Some(result) = main_witness.get_index(3) {
    //     println!("Execution successful! Result: {}", result);
    //     println!("Expected: 15 (5 + 10)");
    // } else {
    //     println!("Execution successful but no result found in witness");
    // }
    
    // println!("Witness contains {} values", main_witness.len());
    
    // // Demonstrate using M31 BlackBoxSolver directly
    // println!("\n=== M31 BlackBoxSolver Demo ===");
    // let m31_solver = M31BlackBoxSolver::new();
    // println!("M31 BlackBoxSolver created: {:?}", m31_solver);
    
    // // Try to use M31 solver with execute_program (this will fail for unsupported operations)
    // println!("Note: M31 BlackBoxSolver returns 'Unsupported' for all black box functions");
    // println!("This is expected behavior for this demo implementation.");
    
    // Ok(())
}

/// Creates a simple ACIR program that adds two field elements
/// This is a minimal example to demonstrate the execute_program function
// fn create_simple_addition_program() -> Program<FieldElement> {
//     // Create a simple circuit that adds two inputs
//     // Input: witness[1] + witness[2] = witness[3]
    
//     let circuit = Circuit {
//         current_witness_index: 4, // We'll use indices 1, 2, 3
//         opcodes: vec![
//             // This would normally contain actual ACIR opcodes
//             // For this demo, we'll create a minimal circuit
//         ],
//         public_parameters: vec![], // No public inputs for this demo
//         return_values: vec![3], // Return the result at witness index 3
//         assert_messages: vec![],
//     };
    
//     Program {
//         functions: vec![circuit],
//         unconstrained_functions: vec![],
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_execution() {
        main(); // Just call main, it returns unit type ()
    }
    
    #[test]
    fn test_m31_hello_world() {
        let result = M31BlackBoxSolver::hello_world();
        assert_eq!(result, "Hello World from M31 BlackBoxSolver!", "Hello world should return the expected string");
    }
} 