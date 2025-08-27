use rogueworks_core::{Plugin, App};

pub struct InteractionsPlugin;
impl Plugin for InteractionsPlugin {
    fn name(&self) -> &'static str { "interactions" }
    fn build(&self, _app: &mut App) {
        tracing::info!("InteractionsPlugin build");
    }
}
