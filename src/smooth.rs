fn sigmoid(x: f32) -> f32 {
  1.0 / (1.0 + (-x).exp())
}

fn dsigmoid(x: f32) -> f32 {
  let exp = (-x).exp();
  let sigmoid = 1.0 / (1.0 + exp);
  exp * sigmoid*sigmoid
}

pub fn smooth(y0: f32, y1: f32, x: f32) -> f32 {
  let dsigmoid = dsigmoid(1.0) / 3.0;
  let h = sigmoid(1.0) - dsigmoid;
  let l = sigmoid(-1.0) + dsigmoid;
  let yscale = (y1 - y0) / (h - l);
  let x = 2.0 * x - 1.0;
  let y = sigmoid(x) - dsigmoid * x*x*x;
  yscale * (y - l) + y0
}

#[cfg(test)]
mod test {
  use super::smooth;

  fn fexpect(expect: f32, actual: f32) -> bool {
    ((actual - expect) / expect).abs() <= 0.01
  }

  #[test]
  fn at_zero() {
    let l = 1231.728;
    let h = 9314.644;
    assert!(fexpect(l, smooth(l, h, 0.0)));
  }

  #[test]
  fn at_one() {
    let l = 1231.728;
    let h = 9314.644;
    assert!(fexpect(h, smooth(l, h, 1.0)));
  }
}

#[cfg(test)]
mod bench {
  extern crate test;

  use super::smooth;

  #[bench]
  fn bench_smooth(bench: &mut test::Bencher) {
    bench.iter(|| {
      test::black_box(smooth(13.0, 17.0, 0.5));
    });
  }
}
