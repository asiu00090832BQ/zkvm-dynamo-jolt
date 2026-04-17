use zkvm_core::{decode_word, Vm};

#[test]
fn executes_mul() {
    let mut vm = Vm::new();
    vm.write_reg(1, 6);
    vm.write_reg(2, 7);
    vm.execute_word(0x0220_81b3).unwrap();
    assert_eq!(vm.read_reg(3), 42);
}
