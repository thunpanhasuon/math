# math-rs

A small 2D math library for games, written in Rust.

## What's in it

- **`vec2`** — 2D vector (`Vec2`): add, subtract, dot product, cross product, length, scale, normalize, and lerp.
- **`mat2`** — 2D matrix (`Mat2`): scale, reflection (swap / flip x / flip y / flip origin), and counter-clockwise rotation.
- **`calculus`** — basic 2D physics (`Physic`): gravity and projectile arc, plus an `Aabb` (axis-aligned bounding box) for simple collision checks.
- **`random`** — `SeedEngine`: a reproducible RNG for world generation paired with a fast RNG for gameplay rolls (critical hits, percentage-based chance checks).
- **`timer`** — `RsTime`: frame delta-time tracking.

## Usage

Add it to your `Cargo.toml`:

```toml
[dependencies]
math-rs = "0.1.0"
```

```rust
use math_rs::vec2::Vec2;

let a = Vec2::new(1.0, 2.0);
let mut b = Vec2::new(3.0, 4.0);
let sum = a.add_vec2(&mut b);
```

## Status

Early / actively developed — the API may still change between versions.

## License

MIT — see [LICENSE](./LICENSE).
