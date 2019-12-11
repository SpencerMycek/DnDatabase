use std::process;

fn main() {
    if let Err(e) = turn_tracker::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
