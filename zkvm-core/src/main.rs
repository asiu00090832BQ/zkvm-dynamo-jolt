fn cmd_verify(path: &str) -> Result<(), Box<dyn Error>> {
    let program = load_program(path)?;
    let proof = prove_program::<Fr>(&program)?;
    verify_program::<Fr>(&program, &proof)?;
    println!("Program verified successfully.");
    Ok(())
}
