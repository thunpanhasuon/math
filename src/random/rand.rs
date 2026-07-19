use rand::{RngExt, TryRng};
use rand::rngs::{SysRng, SmallRng};
use rand::SeedableRng;
use rand_pcg::Pcg64;

pub struct SeedEngine {
    world_gen_rng: Pcg64,   // Seeded, reproducible for maps
    gameplay_rng: SmallRng,  // Fast, non-cryptographic for combat
}

impl SeedEngine {
    pub fn new(world_seed: [u8; 32]) -> Self {
        // 1. Use SysRng to get a completely random seed for general gameplay.
        // SysRng only implements the fallible TryRng interface (it can fail
        // if the OS randomness source is unavailable), so fill the seed
        // bytes directly instead of calling `.random()`.
        let mut seed_source = SysRng;
        let mut gameplay_seed = [0u8; 32];
        seed_source
            .try_fill_bytes(&mut gameplay_seed)
            .expect("failed to read OS randomness");

        Self {
            // 2. Initialize PCG with a specific seed so the world generates the same way every time
            world_gen_rng: Pcg64::from_seed(world_seed),
            // 3. Initialize SmallRng for ultra-fast performance during gameplay
            gameplay_rng: SmallRng::from_seed(gameplay_seed),
        }
    }

    pub fn generate_map_tile(&mut self) -> u32 {
        self.world_gen_rng.random_range(0..100) // Always identical for the same seed
    }

    pub fn roll_critical_hit(&mut self) -> bool {
        self.gameplay_rng.random_bool(0.15) // 15% critical hit chance, super fast
    }

    // General version of roll_critical_hit: pass any percentage chance
    // (0.0..=100.0) instead of a hardcoded 0.15.
    pub fn roll_chance(&mut self, percentage: f32) -> bool {
        let probability = (percentage / 100.0).clamp(0.0, 1.0) as f64;
        self.gameplay_rng.random_bool(probability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_chance_bounds() {
        let mut engine = SeedEngine::new([0u8; 32]);

        // 0% should never hit, 100% should always hit
        assert_eq!(false, engine.roll_chance(0.0));
        assert_eq!(true, engine.roll_chance(100.0));
    }
}
