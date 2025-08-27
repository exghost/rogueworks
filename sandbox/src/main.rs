use rogueworks_core::{App};
use rogueworks_diagnostics::init_tracing;

fn main() {
    init_tracing();
    let _app = App::default();
    println!("Rogueworks sandbox booted (skeleton).");
}
