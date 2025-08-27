# Render Intents & Adapters

Rendering is presentation-agnostic. Systems produce a `RenderFrame`:

```rust
pub enum Glyph { Char(char), TileId(u32) }
pub struct RenderCell { pos: IVec2, glyph: Glyph, fg: Rgb, bg: Rgb, z: i32 }
pub struct RenderFrame { cells: Vec<RenderCell> }
```

Adapters (ASCII, headless, web) translate `RenderFrame` into pixels or DOM.
