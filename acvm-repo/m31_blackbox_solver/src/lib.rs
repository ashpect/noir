use acvm::{BlackBoxFunctionSolver, FieldElement};
use acir::BlackBoxFunc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum M31BlackBoxSolverError {
    #[error("Unsupported black box function: {0:?}")]
    UnsupportedBlackBoxFunction(BlackBoxFunc),
}

/// A simple black box solver for M31 field operations
/// This is a basic implementation that provides a "hello world" function
#[derive(Debug, Clone)]
pub struct M31BlackBoxSolver;

impl M31BlackBoxSolver {
    /// Creates a new M31 black box solver
    pub fn new() -> Self {
        Self
    }

    /// A simple hello world function that returns a constant value
    pub fn hello_world() -> &'static str {
        // Return a simple hello world string
        "Hello World from M31 BlackBoxSolver!"
    }
}

impl Default for M31BlackBoxSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl BlackBoxFunctionSolver<FieldElement> for M31BlackBoxSolver {
    fn pedantic_solving(&self) -> bool {
        false
    }

    fn multi_scalar_mul(
        &self,
        _points: &[FieldElement],
        _scalars_lo: &[FieldElement],
        _scalars_hi: &[FieldElement],
    ) -> Result<(FieldElement, FieldElement, FieldElement), acvm::BlackBoxResolutionError> {
        Err(acvm::BlackBoxResolutionError::Failed(
            BlackBoxFunc::MultiScalarMul,
            "M31 BlackBoxSolver does not support MultiScalarMul".to_string(),
        ))
    }

    fn ec_add(
        &self,
        _input1_x: &FieldElement,
        _input1_y: &FieldElement,
        _input1_infinite: &FieldElement,
        _input2_x: &FieldElement,
        _input2_y: &FieldElement,
        _input2_infinite: &FieldElement,
    ) -> Result<(FieldElement, FieldElement, FieldElement), acvm::BlackBoxResolutionError> {
        Err(acvm::BlackBoxResolutionError::Failed(
            BlackBoxFunc::EmbeddedCurveAdd,
            "M31 BlackBoxSolver does not support EmbeddedCurveAdd".to_string(),
        ))
    }

    fn poseidon2_permutation(
        &self,
        _inputs: &[FieldElement],
        _len: u32,
    ) -> Result<Vec<FieldElement>, acvm::BlackBoxResolutionError> {
        Err(acvm::BlackBoxResolutionError::Failed(
            BlackBoxFunc::Poseidon2Permutation,
            "M31 BlackBoxSolver does not support Poseidon2Permutation".to_string(),
        ))
    }
}
