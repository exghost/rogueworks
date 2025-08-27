# Resources & Events

Core resources:
- `GameTime { tick, delta }`
- `RngResource { seed, rng }`
- `ActionQueue(Vec<Action>)`

Events are typed, single-frame or buffered channels (e.g., `InteractionRequested`, `TurnAdvanced`).
