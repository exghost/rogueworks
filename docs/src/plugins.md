# Plugin System

Plugins encapsulate systems/resources and declare dependencies to control init order.

```rust
pub trait Plugin {
    fn name(&self) -> &'static str;
    fn build(&self, app: &mut App);
    fn depends_on(&self) -> &'static [&'static str] { &[] }
}
```
