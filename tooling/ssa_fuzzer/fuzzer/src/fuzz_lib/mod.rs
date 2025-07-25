pub(crate) mod base_context;
pub(crate) mod block_context;
pub(crate) mod fuzz_target_lib;
pub(crate) mod fuzzer;
pub(crate) mod instruction;
pub(crate) mod options;

pub(crate) const NUMBER_OF_VARIABLES_INITIAL: u32 = 7;
/// Numbers of variables that are predefined in the fuzzer
pub(crate) const NUMBER_OF_PREDEFINED_VARIABLES: u32 = 2;
