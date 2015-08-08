use rand;
use rand::{Rng};

use smooth;

// TODO: Seed parameter
pub fn eval(bucket_size: f32, x: f32) -> f32 {
  let gradient = |seed: u32| {
    let seed = vec!(seed as usize);
    let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed.as_slice());
    let x = rng.next_u32() as i32 as f32;
    let amplitude = 0x80000000 as u32 as f32;
    x / amplitude
  };

  let scaled_x: f32 = x / bucket_size;
  let bucket0 = scaled_x.floor();
  let bucket0 = bucket0 as u32;
  let bucket1 = bucket0 + 1;

  let dx = scaled_x - bucket0 as f32;
  let grad0 = gradient(bucket0);
  let grad1 = gradient(bucket1);

  smooth::smooth(grad0, grad1, dx)
}

#[cfg(test)]
mod bench {
  extern crate test;

  use noise;

  #[bench]
  fn bench_mine(bench: &mut test::Bencher) {
    bench.iter(|| {
      test::black_box(super::eval(1.0, 0.5));
    });
  }

  #[bench]
  fn bench_noise_rs(bench: &mut test::Bencher) {
    let seed = noise::Seed::new(0);
    bench.iter(|| {
      test::black_box(noise::perlin1(&seed, &[13.0]))
    });
  }
}
