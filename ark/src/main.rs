use snark::r1cs::ConstraintSystem;
use ec::models::bn::BnParameters;
use r1cs::alloc::AllocationMode::{Input, Witness, Constant};
use ff::fields::models::Fp256;
use bn254::Fr;

fn prove_sum() {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let input = cs.new_input_variable(|| Ok(Fr::from(42u16))).unwrap();
    // for value in values.iter() {
    //     sum_v += var(&cs, value, Witness);
    // }
    // sum_i.enforce_equal(&sum_v).unwrap();
    // cs.finalize();
    // assert!(cs.is_satisfied().unwrap());
}

fn main() {
    prove_sum();
}
