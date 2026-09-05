use std::env;

fn print_help() {
    println!("VBF Layer 0 diagnostic CLI");
    println!();
    println!("Usage:");
    println!("  vbf version");
    println!("  vbf new-entity-uid");
    println!("  vbf help");
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("version") => println!("vbf {}", env!("CARGO_PKG_VERSION")),
        Some("new-entity-uid") => println!("{}", vbf_types::EntityUid::new()),
        Some("help") | None => print_help(),
        Some(other) => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}
