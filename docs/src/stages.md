# Stages & Schedules

Stages sequence:
1. `Startup` (once)
2. `PreUpdate`
3. `Update`
4. `PostUpdate`
5. `RenderPrep`
6. `Teardown`

Each stage corresponds to a Legion `Schedule`. Plugins register systems into one or more stages.
