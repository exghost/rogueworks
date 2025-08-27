//! Diagnostics setup (tracing subscriber).

use tracing_subscriber::prelude::*;

/// Initialize a simple tracing subscriber for debugging.
pub fn init_tracing() {
    let fmt = tracing_subscriber::fmt::layer().with_target(false);
    tracing_subscriber::registry().with(fmt).init();
}
