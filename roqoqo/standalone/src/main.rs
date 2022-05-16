// Copyright © 2021 HQS Quantum Simulations GmbH.

mod measurement_example;
use measurement_example::measurement_main;

mod teleportation_example;
use teleportation_example::teleportation_main;

mod intro_to_roqoqo;
use intro_to_roqoqo::*;

mod simple_vha_with_roqoqo;
use simple_vha_with_roqoqo::*;

fn main() {
    entangling_circuit_snippet();
    measuring_qubits();
    measuring_observables();
    serialization_quantum_program();
    measurement_main();
    teleportation_main();
    run_simple_vha();
}
