extern crate libspartan;
extern crate merlin;
use libspartan::{Instance, SNARKGens, SNARK};
use merlin::Transcript;

fn main() {
    // specify the size of an R1CS instance
    let num_vars = 1024;
    let num_cons = 1024;
    let num_inputs = 10;
    let num_non_zero_entries = 1024;

    // produce public parameters
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_non_zero_entries);

    // ask the library to produce a synthetic R1CS instance
    let (inst, vars, inputs) = Instance::produce_synthetic_r1cs(num_cons, num_vars, num_inputs);

    // create a commitment to the R1CS instance
    let (com, decomm) = SNARK::encode(&inst, &gens);

    // produce a proof of satisfiability
    let mut proof_transcript = Transcript::new(b"snark_example_1_xx");
    let proof = SNARK::prove(
        &inst,
        &com,
        &decomm,
        vars,
        &inputs,
        &gens,
        &mut proof_transcript,
    );

    // verify the proof of satisfiability
    let mut verify_transcript = Transcript::new(b"snark_example_1_xx");
    assert!(proof
        .verify(&com, &inputs, &mut verify_transcript, &gens)
        .is_ok());

    println!("proof verification successful")
}
