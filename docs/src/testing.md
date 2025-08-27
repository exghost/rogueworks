# Testing Strategy

- Unit tests for pure logic (FOV, carving)
- Property tests with `proptest`
- Integration tests: headless runs with asserted invariants
- Golden tests: ASCII/JSON snapshots
- CI: clippy, fmt, test matrix
