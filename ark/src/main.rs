/// r1cs-std/target/doc/src/ark_relations/r1cs/constraint_system.rs.html#31-74
use snark::r1cs::ConstraintSystem;
use snark::r1cs::SynthesisMode;
use snark::lc;
use bn254::Fr;

fn prove_sum(input: Fr) {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let ret = input.clone() + input;
    cs.set_mode(SynthesisMode::Setup);
    // Obtain a variable representing a new private witness input.
    let a = cs.new_input_variable(|| Ok(input)).unwrap();
    let b = cs.new_input_variable(|| Ok(input)).unwrap();
    let c = cs.new_witness_variable(|| Ok(ret)).unwrap();
    cs.enforce_constraint(
        lc!() + a,
        lc!() + (ret, b),
        lc!() + c
    ).unwrap();

    cs.new_lc(lc!() + a).unwrap();
    cs.inline_all_lcs();
    cs.finalize();
    println!("{:?}", cs.clone());
    println!("================================");
    println!("{:?}", cs.to_matrices().unwrap());

}

fn main() {
    prove_sum(Fr::from(42u16));
}
