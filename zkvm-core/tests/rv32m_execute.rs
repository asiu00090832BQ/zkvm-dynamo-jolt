use zkvm_core::Zkvm;

#[test]
fn executes_mul() {
    let program = vec![0x022081b3, 0x00100073];
    let mut vm = Zkvm::new(program);
    vm.set_register(1, 10).unwrap();
    vm.set_register(2, 20).unwrap();
    vm.run().unwrap();
    assert_eq!(vm.register(3).unwrap(), 200);
}
