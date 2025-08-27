use rogueworks_core::{Plugin, App};

pub struct FovPlugin;
impl Plugin for FovPlugin {
    fn name(&self) -> &'static str { "fov" }
    fn build(&self, _app: &mut App) {
        tracing::info!("FovPlugin build");
    }
}
