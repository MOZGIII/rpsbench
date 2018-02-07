#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate futures_await as futures;

use futures::prelude::*;

#[async]
fn work() -> Result<(), ()> {
  println!("Hello, world!");
  Ok(())
}

#[async]
fn run() -> Result<(), ()> {
  loop {
    work();
  }
}

fn main() {
  run().wait().unwrap()
}
