use snark::r1cs::ConstraintSystem;
use snark::r1cs::LinearCombination;
use ec::models::bn::BnParameters;
use r1cs::alloc::AllocationMode::{Input, Witness, Constant};
use ff::fields::models::Fp256;
use bn254::Fr;
use std::ops::Mul;

fn prove_sum() {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let input = cs.new_input_variable(|| Ok(Fr::from(42u16))).unwrap();
    let mut lc = LinearCombination::<Fr>::zero();
    lc = lc + input;
    lc = lc.mul(Fr::from(3u16));
    cs.new_lc(lc).unwrap();
    cs.finalize();
    println!("{:?}", cs);
}

fn main() {
    prove_sum();
}
