use std::error::Error;

fn main() {
    if let Err(e) = read_excel() {
        eprintln!("there is some problem {}", e);
        std::process::exit(1);
    }
}

fn read_excel() -> Result<(), Box<dyn Error>> {
    return Ok(());
}
