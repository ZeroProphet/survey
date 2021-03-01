use snark::r1cs::ConstraintSystem;
use snark::r1cs::LinearCombination;
use snark::r1cs::SynthesisMode;
use bn254::Fr;

fn prove_sum() {
    let cs = ConstraintSystem::<Fr>::new_ref();
    cs.set_mode(SynthesisMode::Setup);
    // Obtain a variable representing a new private witness input.
    let input = cs.new_input_variable(|| Ok(Fr::from(42u16))).unwrap();
    let lc = LinearCombination::<Fr>::zero();
    cs.new_lc(lc + input).unwrap();
    cs.finalize();
    println!("{:?}", cs.clone());
    println!("================================");
    println!("{:?}", cs.to_matrices().unwrap());

}

fn main() {
    prove_sum();
}
