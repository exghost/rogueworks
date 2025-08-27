use rogueworks_core::{App, Stage};

/// Run a simple headless loop that executes registered schedules in order.
pub fn run_headless(mut app: App, ticks: u64) {
    for _ in 0..ticks {
        for (stage, _sched) in &app.schedules {
            match stage {
                Stage::Startup => {}     // stub: would run once normally
                Stage::PreUpdate => {}
                Stage::Update => {}
                Stage::PostUpdate => {}
                Stage::RenderPrep => {}
                Stage::Teardown => {}
            }
        }
    }
}
