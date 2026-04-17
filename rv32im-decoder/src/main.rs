use std::process::ExitCode;

use rv32im_decoder::decode_hex;

fn main() -> ExitCode {
    let mut args = std::env::args();
    let program = args.next().unwrap_or_else(|| "rv32im-decoder".to_owned());

    let Some(word) = args.next() else {
        eprintln!("usage: {program} <hex-word>");
        return ExitCode::from(2);
    };

    match decode_hex(&word) {
        Ok(instruction) => {
            println!("{instruction:#?}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}
