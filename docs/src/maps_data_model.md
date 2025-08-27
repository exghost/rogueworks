# Map Data Model

```rust
pub struct Tile { pub kind: TileKind, pub flags: BitFlags<TileFlag> }
pub struct Map { pub size: IVec2, pub tiles: Vec<Tile>, pub rooms: Vec<Rect>, pub entrances: Vec<IVec2> }
```
