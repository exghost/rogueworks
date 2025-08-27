# Rogueworks Docs (mdBook)

## Prereqs
- Install mdBook: https://rust-lang.github.io/mdBook/guide/installation.html

## Build locally
```bash
cd docs
mdbook serve -n 127.0.0.1 -p 3000
# or build static site
mdbook build
```

## Structure
- `book.toml`: configuration
- `src/`: markdown chapters (see `SUMMARY.md`)

You can copy this `docs/` folder into your repository root.
