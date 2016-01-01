extern crate arrayvec;
extern crate rand;
extern crate collect_slice;
extern crate num;

pub mod decoder;
pub mod consts;

mod allocs;
mod chunk;
mod coefs;
mod descramble;
mod enhance;
mod errors;
mod gain;
mod noise;
mod params;
mod prev;
mod scan;
mod spectral;
mod unvoiced;
mod voiced;
mod window;
