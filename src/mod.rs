#![feature(convert)]

extern crate gl;
#[macro_use]
extern crate log;
extern crate noise;
extern crate rand;
extern crate sdl2;
extern crate stopwatch;
extern crate yaglw;

mod main;

pub fn main() {
  main::main();
}
