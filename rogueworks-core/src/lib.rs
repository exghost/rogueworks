//! Core ECS kernel, plugin API, resources, events, and render intents.

use legion::{World, Resources, Schedule};

/// Engine stages executed each tick.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Stage { Startup, PreUpdate, Update, PostUpdate, RenderPrep, Teardown }

/// Top-level application container.
pub struct App {
    pub world: World,
    pub resources: Resources,
    pub schedules: Vec<(Stage, Schedule)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: World::default(),
            resources: Resources::default(),
            schedules: Vec::new(),
        }
    }
}

/// Plugin interface to register systems/resources.
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn build(&self, app: &mut App);
    fn depends_on(&self) -> &'static [&'static str] { &[] }
}

/// RNG and time resources.
#[derive(Debug, Clone)]
pub struct GameTime { pub tick: u64 }
impl Default for GameTime { fn default() -> Self { Self { tick: 0 } } }

#[derive(Debug, Clone)]
pub struct RngSeed(pub u64);

/// High-level input actions.
#[derive(Debug, Clone)]
pub enum Action { Move(Dir), Wait, Interact, ToggleDebug }
#[derive(Debug, Copy, Clone)]
pub enum Dir { North, South, East, West }

#[derive(Debug, Default)]
pub struct ActionQueue(pub Vec<Action>);

/// Presentation-agnostic render intents.
#[derive(Debug, Clone)]
pub enum Glyph { Char(char), TileId(u32) }

#[derive(Debug, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Debug, Clone)]
pub struct RenderCell { pub x: i32, pub y: i32, pub z: i32, pub glyph: Glyph, pub fg: Rgb, pub bg: Rgb }

#[derive(Debug, Default, Clone)]
pub struct RenderFrame { pub cells: Vec<RenderCell> }

impl App {
    /// Register a schedule for a stage.
    pub fn add_schedule(&mut self, stage: Stage, schedule: Schedule) {
        self.schedules.push((stage, schedule));
    }
}
