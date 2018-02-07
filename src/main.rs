#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate futures_await as futures;
extern crate futures_cpupool;

use futures::prelude::*;
use futures_cpupool::CpuPool;
use std::*;

#[async]
fn work() -> Result<(), ()> {
  println!("Hello from {:?}", thread::current().id());
  Ok(())
}

#[async]
fn run() -> Result<(), ()> {
  let pool = CpuPool::new(4);

  loop {
    await!(pool.spawn(work()));
  }
}

fn main() {
  run().wait().unwrap()
}
