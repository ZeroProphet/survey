use snark::r1cs::ConstraintSystem;
use snark::r1cs::SynthesisMode;
use snark::lc;
use bn254::Fr;

fn prove_sum(input: Fr) {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let ret = input.clone() + input;
    cs.set_mode(SynthesisMode::Setup);
    // Obtain a variable representing a new private witness input.
    let input_var = cs.new_input_variable(|| Ok(Fr::from(42u16))).unwrap();
    let output_var = cs.new_witness_variable(|| Ok(ret)).unwrap();
    cs.enforce_constraint(
        lc!() + input_var,
        lc!() + input_var,
        lc!() + output_var
    ).unwrap();

    cs.new_lc(lc!() + input_var).unwrap();
    cs.inline_all_lcs();
    cs.finalize();
    println!("{:?}", cs.clone());
    println!("================================");
    println!("{:?}", cs.to_matrices().unwrap());

}

fn main() {
    prove_sum(Fr::from(42u16));
}
