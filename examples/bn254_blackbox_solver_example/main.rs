use bn254_blackbox_solver::{
    Bn254BlackBoxSolver, 
    derive_generators,
};
use acir::acir_field::GenericFieldElement;
use acir::AcirField;
use acvm_blackbox_solver::BlackBoxFunctionSolver;
use ark_ec::AffineRepr;

fn main() {
    println!("=== BN254 BlackBox Solver Multi-Scalar Multiplication Example ===\n");

    // Create a solver instance (pedantic_solving = false for this example)
    let solver = Bn254BlackBoxSolver(false);
    
    // Example 1: Simple scalar multiplication with generator point
    println!("Example 1: Simple scalar multiplication");
    example_simple_scalar_mul(&solver);
    
    println!("\n{}", "=".repeat(60));
    println!();
    
    // // Example 2: Multi-scalar multiplication with multiple points
    // println!("Example 2: Multi-scalar multiplication");
    // example_multi_scalar_mul(&solver);
    
    // println!("\n{}", "=".repeat(60));
    // println!();
    
    // // Example 3: Curve point addition
    // println!("Example 3: Curve point addition");
    // example_curve_addition(&solver);
}

fn example_simple_scalar_mul(solver: &Bn254BlackBoxSolver) {
    // Get a generator point
    let generators = derive_generators("DEFAULT_DOMAIN_SEPARATOR".as_bytes(), 1, 0);
    let generator = generators[0];
    
    // Convert generator to field elements (x, y, is_infinite)
    let generator_x = GenericFieldElement::<ark_bn254::Fr>::from_repr(generator.x().unwrap());
    let generator_y = GenericFieldElement::<ark_bn254::Fr>::from_repr(generator.y().unwrap());
    let is_infinite = GenericFieldElement::<ark_bn254::Fr>::zero(); // Generator is not infinite
    

    println!("Generator point:");
    println!("  x: {}", generator_x);
    println!("  y: {}", generator_y);
    println!("  is_infinite: {}", is_infinite);
    
    // Create points array (single point)
    let points = vec![generator_x, generator_y, is_infinite];
    
    // Scalar = 2 (split into low and high limbs)
    let scalar_lo = vec![GenericFieldElement::<ark_bn254::Fr>::from(2u128)];
    let scalar_hi = vec![GenericFieldElement::<ark_bn254::Fr>::zero()];
    
    println!("\nScalar: 2");
    println!("  low limb: {}", scalar_lo[0]);
    println!("  high limb: {}", scalar_hi[0]);
    
    // Perform multi-scalar multiplication
    match solver.multi_scalar_mul(&points, &scalar_lo, &scalar_hi) {
        Ok((result_x, result_y, result_infinite)) => {
            println!("\nResult (2 * generator):");
            println!("  x: {}", result_x.to_hex());
            println!("  y: {}", result_y.to_hex());
            println!("  is_infinite: {}", result_infinite);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn example_multi_scalar_mul(solver: &Bn254BlackBoxSolver) {
    // Get two generator points
    let generators = derive_generators("test_domain".as_bytes(), 2, 0);
    
    let mut points = Vec::new();
    let mut scalars_lo = Vec::new();
    let mut scalars_hi = Vec::new();
    
    for (i, generator) in generators.iter().enumerate() {
        let x = GenericFieldElement::<ark_bn254::Fr>::from_repr(generator.x().unwrap());
        let y = GenericFieldElement::<ark_bn254::Fr>::from_repr(generator.y().unwrap());
        let is_infinite = GenericFieldElement::<ark_bn254::Fr>::zero();
        
        points.extend_from_slice(&[x, y, is_infinite]);
        
        // Scalar for this point: i + 1
        let scalar = GenericFieldElement::<ark_bn254::Fr>::from((i + 1) as u128);
        scalars_lo.push(scalar);
        scalars_hi.push(GenericFieldElement::<ark_bn254::Fr>::zero());
        
        println!("Point {}:", i + 1);
        println!("  x: {}", x.to_hex());
        println!("  y: {}", y.to_hex());
        println!("  scalar: {}", scalar);
    }
    
    println!("\nComputing: 1*G1 + 2*G2");
    
    // Perform multi-scalar multiplication
    match solver.multi_scalar_mul(&points, &scalars_lo, &scalars_hi) {
        Ok((result_x, result_y, result_infinite)) => {
            println!("\nResult:");
            println!("  x: {}", result_x.to_hex());
            println!("  y: {}", result_y.to_hex());
            println!("  is_infinite: {}", result_infinite);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn example_curve_addition(solver: &Bn254BlackBoxSolver) {
    // Get a generator point
    let generators = derive_generators("test_domain".as_bytes(), 1, 0);
    let generator = generators[0];
    
    let point1_x = GenericFieldElement::<ark_bn254::Fr>::from_repr(generator.x().unwrap());
    let point1_y = GenericFieldElement::<ark_bn254::Fr>::from_repr(generator.y().unwrap());
    let point1_infinite = GenericFieldElement::<ark_bn254::Fr>::zero();
    
    // Add the generator to itself (equivalent to 2*generator)
    let point2_x = point1_x;
    let point2_y = point1_y;
    let point2_infinite = GenericFieldElement::<ark_bn254::Fr>::zero();
    
    println!("Point 1 (generator):");
    println!("  x: {}", point1_x.to_hex());
    println!("  y: {}", point1_y.to_hex());
    
    println!("\nPoint 2 (same generator):");
    println!("  x: {}", point2_x.to_hex());
    println!("  y: {}", point2_y.to_hex());
    
    // Perform curve addition
    match solver.ec_add(
        &point1_x, &point1_y, &point1_infinite,
        &point2_x, &point2_y, &point2_infinite,
    ) {
        Ok((result_x, result_y, result_infinite)) => {
            println!("\nResult (G + G = 2G):");
            println!("  x: {}", result_x.to_hex());
            println!("  y: {}", result_y.to_hex());
            println!("  is_infinite: {}", result_infinite);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_scalar_mul() {
        let solver = Bn254BlackBoxSolver(false);
        example_simple_scalar_mul(&solver);
    }
    
    #[test]
    fn test_multi_scalar_mul() {
        let solver = Bn254BlackBoxSolver(false);
        example_multi_scalar_mul(&solver);
    }
    
    #[test]
    fn test_curve_addition() {
        let solver = Bn254BlackBoxSolver(false);
        example_curve_addition(&solver);
    }
} 