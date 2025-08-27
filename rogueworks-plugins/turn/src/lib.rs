use rogueworks_core::{Plugin, App};

#[derive(Debug, Copy, Clone)]
pub enum TurnState { Player, AI, World }

pub struct TurnPlugin;
impl Plugin for TurnPlugin {
    fn name(&self) -> &'static str { "turn" }
    fn build(&self, _app: &mut App) {
        tracing::info!("TurnPlugin build");
    }
}
