use ark_ff::{Fp, MontBackend, MontConfig};
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ec::CurveConfig;

#[derive(MontConfig)]
#[modulus = "2147483647"] // p = 2^31 - 1
#[generator = "7"]
pub struct M31Config;

pub type M31 = Fp<MontBackend<M31Config, 1>, 1>;

pub struct MyBN128OverM31;

impl CurveConfig for MyBN128OverM31 {
    type BaseField = M31;
    type ScalarField = M31; // Just a placeholder; no security.
}

impl SWCurveConfig for MyBN128OverM31 {
    const COEFF_A: M31 = M31::from(0u32);
    const COEFF_B: M31 = M31::from(3u32);

    const GENERATOR: Affine<Self> = Affine::new_unchecked(
        M31::from(1u32), // example x-coordinate for G
        M31::from(2u32), // example y-coordinate for G
    );
}

type MyAffine = Affine<MyBN128OverM31>;

fn main() {
    let g = MyAffine::generator();
    println!("Generator point on BN128-like curve over M31: {:?}", g);
}
