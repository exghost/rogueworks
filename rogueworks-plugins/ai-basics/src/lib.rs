use rogueworks_core::{Plugin, App};

pub struct AiBasicsPlugin;
impl Plugin for AiBasicsPlugin {
    fn name(&self) -> &'static str { "ai-basics" }
    fn build(&self, _app: &mut App) {
        tracing::info!("AiBasicsPlugin build");
    }
}
