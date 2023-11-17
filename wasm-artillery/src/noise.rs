/// Perlin Noise generator that outputs 1D Perlin noise
/// 
/// 
use rand::prelude::*;
#[derive(Clone)]
pub struct PerlinNoise {
    perm: [usize; 512],
    octaves: usize,
    fallout: f64,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut perm = [0; 512];
        let mut rng = thread_rng();

        for i in 0..256 {
            perm[i] = i;
        }

        for i in 0..256 {
            let j = rng.gen_range(0, 256) & 0xFF;
            let t = perm[j];

            perm[j] = perm[i];
            perm[i] = t;
        }

        for i in 0..256 {
            perm[i + 256] = perm[i];
        }

        PerlinNoise {
            perm,
            octaves: 4,
            fallout: 0.5,
        }
    }


    /// Perlin Noise in 1D
    pub fn get(&self, x: f64) -> f64 {
        let mut effect = 1.0;
        let mut k = 1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * ((1.0 + self.noise1d(k * x)) / 2.0);
            k *= 2.0
        }

        sum
    }

    fn noise1d(&self, mut x: f64) -> f64 {
        let x0 = (x.floor() as usize) & 255;

        x -= x.floor();

        let fx = (3.0 - 2.0 * x) * x * x;
        lerp(
            fx,
            grad1d(self.perm[x0], x),
            grad1d(self.perm[x0 + 1], x + 1.0),
        )
    }
}


fn grad1d(hash: usize, x: f64) -> f64 {
    if (hash & 1) == 0 {
        -x
    } else {
        x
    }
}

// Linear Interpolate
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}
// Fade function as defined by Ken Perlin.  This eases coordinate values
// so that they will "ease" towards integral values.  This ends up smoothing
// the final output.