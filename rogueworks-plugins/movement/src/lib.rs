use rogueworks_core::{Plugin, App};

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn name(&self) -> &'static str { "movement" }
    fn build(&self, _app: &mut App) {
        // Register movement systems here (stub for now).
        tracing::info!("MovementPlugin build");
    }
}
