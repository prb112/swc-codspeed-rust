pub mod codspeed;

#![feature(asm_experimental_arch)]
#[cfg(unix)]
pub mod fifo;

mod macros;
mod measurement;
mod request;
mod shared;
pub mod utils;
pub mod walltime_results;
