use r1cs::values;
use r1cs::GadgetBuilder;
use r1cs::Expression;
use r1cs::Bn128;
use r1cs::Element;
use r1cs::Field;
use r1cs::Wire;
use std::fs::File;
use std::collections::hash_set::HashSet;
use r1cs_zkinterface::write_circuit_and_witness;


fn main() {
    // Create a gadget which takes a single input, x, and computes x*x*x.
    let mut builder = GadgetBuilder::<Bn128>::new();
    let x = builder.wire();
    let x_exp = Expression::from(x);
    let x_squared = builder.product(&x_exp, &x_exp);
    let x_cubed = builder.product(&x_squared, &x_exp);
    let gadget = builder.build();

    // This structure maps wires to their (field element) values. Since
    // x is our input, we will assign it a value before executing the
    // gadget. Other wires will be computed by the gadget.
    let mut values = values!(x => 5u8.into());

    // Execute the gadget and assert that all constraints were satisfied.
    let constraints_satisfied = gadget.execute(&mut values);
    assert!(constraints_satisfied);

    // Check the result.
    assert_eq!(Element::from(125u8), x_cubed.evaluate(&values));
    let mut wires = HashSet::<Wire>::new();
    wires.insert(x);
    let mut buffer = File::create("test.zkif").unwrap();
    write_circuit_and_witness(&gadget, &values, &wires, &mut buffer);
}
