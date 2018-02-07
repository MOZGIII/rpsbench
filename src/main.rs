#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate futures_await as futures;
extern crate futures_cpupool;

use futures::prelude::*;
use futures_cpupool::CpuPool;
use std::*;

#[async]
fn work() -> Result<(), ()> {
  loop {
    println!("Hello from {:?}", thread::current().id());
  }
}

#[async]
fn run() -> Result<(), ()> {
  let pool = CpuPool::new_num_cpus();

  loop {
    pool.spawn(work());
  }
}

fn main() {
  run().wait().unwrap()
}
